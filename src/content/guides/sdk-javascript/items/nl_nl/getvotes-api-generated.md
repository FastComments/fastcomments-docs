## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |

## Respons

Retourneert: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getVotes Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3c_prod';
const urlId: string = '/news/2026/typescript-ecosystem-update';
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
// Als er een optionele parameter bestond, bijvoorbeeld includeHidden, zou die als volgt gebruikt kunnen worden:
// const votesWithHidden: GetVotes200Response = await getVotes(tenantId, urlId, { includeHidden: true });
[inline-code-end]

---