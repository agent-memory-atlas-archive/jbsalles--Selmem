// Session isolation checks with dummy keys; never contacts an external LLM.
import assert from 'node:assert/strict';
const base=process.argv[2]?.replace(/\/$/,'');
if(!base)throw new Error('Usage: node deploy/test_public_demo.mjs http://localhost:10000');
async function request(path,body,options={}){
  const headers={'Content-Type':'application/json'};
  if(options.cookie)headers.Cookie=options.cookie;
  if(options.secure)headers['X-Forwarded-Proto']='https';
  if(options.session!==undefined)headers['X-Selmem-Session']=options.session;
  const r=await fetch(base+path,{method:body===undefined?'GET':'POST',headers,body:body===undefined?undefined:JSON.stringify(body),signal:AbortSignal.timeout(10000)});
  return {status:r.status,headers:r.headers,text:await r.text()};
}
async function api(path,body,cookie){const r=await request(path,body,{cookie});assert.equal(r.status,200,`Unexpected status for ${path}`);return JSON.parse(r.text);}
const ar=await request('/session',undefined,{secure:true});const br=await request('/session');assert.equal(ar.status,200);assert.equal(br.status,200);
const ac=ar.headers.get('set-cookie'),bc=br.headers.get('set-cookie');for(const marker of ['HttpOnly','SameSite=Strict','Secure'])assert.ok(ac.includes(marker));
const a=ac.split(';')[0],b=bc.split(';')[0];assert.notEqual(a,b);const as=JSON.parse(ar.text),bs=JSON.parse(br.text);assert.equal(as.memory_lifetime_hours,24);assert.notEqual(as.session_id,bs.session_id);
assert.equal(JSON.parse((await request('/session',undefined,{session:as.session_id})).text).session_id,as.session_id);
assert.equal((await request('/stats',undefined,{session:'forged'})).status,401);
assert.deepEqual(await api('/health'),{ok:true,public_demo:true});
for(const path of ['/stats','/profile']){assert.equal((await request(path)).status,401);assert.equal((await request(path,undefined,{cookie:'selmem_session=forged'})).status,401);}
assert.equal((await request('/live',{event:'unauthenticated write'})).status,401);assert.match((await request('/')).text,/(Demo|<html)/);
assert.equal((await api('/session',undefined,a)).public_demo,true);
await api('/profile',{name:'visitor-A'},a);assert.equal((await api('/profile',undefined,a)).name,'visitor-A');assert.notEqual((await api('/profile',undefined,b)).name,'visitor-A');
assert.equal((await api('/live',{event:'The appointment is Tuesday in room B.',channel:'world'},a)).kept,true);
assert.equal((await api('/book',undefined,a)).traces.length,1);assert.equal((await api('/book',undefined,b)).traces.length,0);
const sa=await api('/stats',undefined,a),sb=await api('/stats',undefined,b);assert.equal(sa.name,'visitor-A');assert.notEqual(sb.name,'visitor-A');assert.equal(sa.traces,1);assert.equal(sb.traces,0);
for(const [stats,cookie] of [[sa,a],[sb,b]]){const book=await api('/book',undefined,cookie);for(const counter of ['traces','axioms','archives'])assert.equal(stats[counter],book[counter].length);}
assert.deepEqual(await api('/health',undefined,a),{ok:true,public_demo:true});await api('/profile',{time_scale:150},a);assert.equal((await api('/profile',undefined,a)).time_scale,150);assert.equal((await api('/profile',undefined,b)).time_scale,24);
const sleepa=await api('/sleep',{},a);
assert.equal(sleepa.hours_advanced,150);assert.equal(sleepa.time_scale,150);
assert.equal((await api('/book',undefined,b)).traces.length,0,'A sleeping must not add memories to B');
const sleepb=await api('/sleep',{},b);
assert.equal(sleepb.hours_advanced,24);assert.equal(sleepb.time_scale,24);
// B now has its own autobiographical sleep memory, but none of A's events.
const bookBAfterSleep=await api('/book',undefined,b);
assert.equal(bookBAfterSleep.traces.length,1);
assert.equal(bookBAfterSleep.traces[0].schema,'night-recollection');
assert.equal(bookBAfterSleep.traces[0].channel,'self');
assert.equal(bookBAfterSleep.archives.length,1);
assert.equal(bookBAfterSleep.archives[0].source,'sleep');
assert.equal((await api('/llm',undefined,a)).has_key,false);assert.equal((await request('/turn',{text:'Hello'},{cookie:a})).status,400);assert.ok((await api('/llm',undefined,a)).plugs.every(p=>!p.has_key));
await api('/llm',{plug:'gpt'},a);assert.equal((await api('/llm',undefined,a)).has_key,false);assert.equal((await request('/turn',{text:'Hello'},{cookie:a})).status,400);
const ka='dummy-visitor-A-key-never-used',kb='dummy-visitor-B-key-never-used';await api('/llm',{plug:'gpt',api_key:ka},a);assert.equal((await api('/llm',undefined,a)).has_key,true);assert.equal((await api('/llm',undefined,b)).has_key,false);
assert.ok(!(await request('/llm',undefined,{cookie:a})).text.includes(ka));assert.ok(!(await request('/book',undefined,{cookie:a})).text.includes(ka));assert.equal(JSON.parse((await request('/stats',undefined,{cookie:b,session:as.session_id})).text).name,'visitor-A');
await api('/llm',{plug:'grok'},b);await api('/llm',{api_key:kb},b);assert.equal((await api('/llm',undefined,b)).has_key,true);assert.equal((await api('/llm',undefined,a)).plug,'gpt');
await api('/llm',{plug:'grok'},a);assert.equal((await api('/llm',undefined,a)).has_key,false);assert.equal((await api('/llm',undefined,b)).has_key,true);assert.equal((await request('/llm',{url:'http://127.0.0.1/private'},{cookie:a})).status,400);
await api('/llm',{clear:true},a);assert.equal((await api('/llm',undefined,a)).attached,false);assert.equal((await api('/llm',undefined,b)).has_key,true);assert.equal((await request('/turn',{text:'Hello'},{cookie:a})).status,400);
assert.deepEqual(await api('/book',undefined,b),bookBAfterSleep,'LLM configuration changes must preserve B\'s own memories');
console.log('Public demo passed: sessions, books, profiles, keys and providers remain isolated.');
