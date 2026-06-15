## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComment200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6f1a2b';
const commentId: string = 'cmt_4d9e8f';
const includeReplies: boolean | undefined = true; // esempio di parametro opzionale (non passato a getComment)
const result: GetComment200Response = await getComment(tenantId, commentId);
console.log('Fetched comment for tenant:', tenantId, 'comment id:', commentId);
console.log('API response received:', result);
[inline-code-end]

---