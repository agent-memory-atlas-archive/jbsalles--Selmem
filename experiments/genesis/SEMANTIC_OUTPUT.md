# Structured semantic interpretation

Semantic interpretation requests specify all required annotation fields and numeric types through a JSON schema. Endpoints that do not support the required response format fail explicitly. Structured formatting does not certify semantic accuracy.

The decoder accepts finite in-range numeric strings without changing their value. Nulls, booleans, arrays, objects, empty/nonnumeric strings and out-of-range values are invalid. No default or clamped numeric annotation is invented.

Invalid returned annotations may be requested again with the identical request, up to three annotation attempts. These attempts and transport retries share the cumulative HTTP budget. Invalid annotations never enter the semantic cache. Corrupt caches stop collection; they are not silently replaced with a new paid answer. A terminal failure prevents committing the candidate memory state.

Raw responses and validation errors remain audit evidence. Use the retained protocol's normal resume command after diagnosis, with unchanged inputs/settings/build. See [reproduction rules](../REPRODUCIBILITY.md).
