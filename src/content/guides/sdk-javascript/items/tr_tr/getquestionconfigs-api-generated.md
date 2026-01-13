## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| skip | number | Hayır |  |

## Yanıt

Döndürür: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigs200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getQuestionConfigs Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-42a7b9';
const firstPage: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
const secondPage: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, 50);
[inline-code-end]

---