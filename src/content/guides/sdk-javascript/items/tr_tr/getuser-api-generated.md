## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getUser Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-publishing-42";
const userIdOptional: string | undefined = "user_9d7b4c"; // bazı akışlarda undefined olabilir (isteğe bağlı)
const id: string = userIdOptional ?? "user_default_0001";
const result: GetUser200Response = await getUser(tenantId, id);
console.log(result);
[inline-code-end]