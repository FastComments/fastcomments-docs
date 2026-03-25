## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getTenant Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f4b2c1a";
const idOverride: string | undefined = undefined; // isteğe bağlı geçersiz kılma, mevcutsa
const id: string = idOverride ?? "site_3e7a6b2f";
const response: GetTenant200Response = await getTenant(tenantId, id);
console.log(response);
[inline-code-end]

---