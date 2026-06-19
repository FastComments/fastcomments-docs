---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |

## Risposta

Restituisce: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_8421';
const urlId: string | undefined = 'posts/2026/06/typescript-api-examples';
const votes: GetVotesResponse = await getVotes(tenantId, urlId!);
[inline-code-end]

---