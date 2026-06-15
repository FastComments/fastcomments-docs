## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |

## Respons

Retourneert: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUser200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getVotesForUser Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9b8f7c6d';
const urlId: string = 'articles/product-update-2026';
const userId: string = 'user_c12345';
const anonUserId: string = 'anon_7f4e2a';
const votes: GetVotesForUser200Response = await getVotesForUser(tenantId, urlId, userId, anonUserId);
[inline-code-end]

---