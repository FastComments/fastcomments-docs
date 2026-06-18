## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Respons

Retourneert: [`GetComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComment200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getComment Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6f1a2b';
const commentId: string = 'cmt_4d9e8f';
const includeReplies: boolean | undefined = true; // voorbeeld van optionele parameter (niet doorgegeven aan getComment)
const result: GetComment200Response = await getComment(tenantId, commentId);
console.log('Fetched comment for tenant:', tenantId, 'comment id:', commentId);
console.log('API response received:', result);
[inline-code-end]

---