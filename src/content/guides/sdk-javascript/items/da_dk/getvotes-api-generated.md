## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |

## Svar

Returnerer: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getVotes Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-42c-eu';
const urlId: string = 'article-7f9b';
const includeMetadata: boolean | undefined = true;
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
[inline-code-end]

---