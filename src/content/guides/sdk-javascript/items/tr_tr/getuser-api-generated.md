## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Dönüş: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getUser Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const idSuffix: string | undefined = undefined;
const tenantId: string = "acme-enterprises";
const id: string = idSuffix ?? "user_98765";
const response: GetUser200Response = await getUser({ tenantId, id });
[inline-code-end]

---