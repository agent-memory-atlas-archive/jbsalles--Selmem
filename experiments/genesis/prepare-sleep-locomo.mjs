// Official text release -> deterministic derived pilot. No LLM and no Python.
import fs from 'node:fs';
import crypto from 'node:crypto';
const [dataset,out]=process.argv.slice(2);
if(!dataset||!out)throw Error('Usage: node prepare-sleep-locomo.mjs DATASET.json OUT.json');
if(fs.existsSync(out))throw Error('Output must be fresh');
const bytes=fs.readFileSync(dataset),sha=crypto.createHash('sha256').update(bytes).digest('hex');
if(sha!=='79fa87e90f04081343b8c8debecb80a9a6842b76a7aa537dc9fdf651ea698ff4')throw Error('Dataset differs from pinned official release');
const data=JSON.parse(bytes),histories=[];
for(const sample of data.slice(0,2)){
 const events=[];
 for(let session=1;session<=4;session++)for(const turn of sample.conversation[`session_${session}`])events.push({id:`${sample.sample_id}:${turn.dia_id}`,dia_id:turn.dia_id,external:true,text:`Conversation on ${sample.conversation[`session_${session}_date_time`]}: ${turn.speaker} said: ${turn.text}`});
 const ids=new Set(events.map(e=>e.dia_id));
 const eligible=sample.qa.map((q,i)=>({...q,index:i})).filter(q=>[1,2,4].includes(q.category)&&q.answer!==undefined&&q.evidence?.length&&q.evidence.every(id=>ids.has(id)));
 const selected=[],seen=new Set();
 for(const type of ['multi-evidence','temporal','factual']){
  const panel=eligible.filter(q=>type==='multi-evidence'?q.evidence.length>1:type==='temporal'?q.category===2:q.category!==2);
  const take=panel.filter(q=>!seen.has(q.index)).slice(0,2);
  if(take.length!==2)throw Error(`Insufficient ${type} questions in ${sample.sample_id}`);
  for(const q of take){seen.add(q.index);selected.push({...q,type});}
 }
 const probes=selected.map((q,i)=>{
  const answer=String(q.answer),pool=[...new Set(data.flatMap(s=>s.qa.filter(x=>x.category===q.category&&x.answer!==undefined).map(x=>String(x.answer))))].filter(a=>a.toLowerCase()!==answer.toLowerCase()&&!a.toLowerCase().includes(answer.toLowerCase())&&!answer.toLowerCase().includes(a.toLowerCase()));
  const values=pool.slice(i*3,i*3+3);if(values.length!==3)throw Error('Not enough distractors');values.splice(i%4,0,answer);
  const options=values.map((v,j)=>({id:`a${j}`,text:v}));options.push({id:'unknown',text:'The supplied memories are insufficient to answer.'});
  return {id:`${sample.sample_id}:q${q.index}`,kind:q.type,category:q.category,question:q.question,evidence_ids:q.evidence,reference_answer:answer,expected:`a${i%4}`,options};
 });
 histories.push({id:sample.sample_id,events,probes});
}
const fixture={version:'sleep-locomo-1',dataset_url:'https://raw.githubusercontent.com/snap-research/locomo/main/data/locomo10.json',dataset_sha256:sha,scope:'Derived multiple-choice pilot, first two conversations, first four sessions each. Not the official LoCoMo benchmark. Distractors need review; faithful qualification is mandatory. No images or commonsense/adversarial questions.',histories};
fs.writeFileSync(out,JSON.stringify(fixture,null,2)+'\n');console.log(JSON.stringify({histories:histories.length,events:histories.reduce((n,h)=>n+h.events.length,0),questions:12,fixture_sha256:crypto.createHash('sha256').update(fs.readFileSync(out)).digest('hex')}));
