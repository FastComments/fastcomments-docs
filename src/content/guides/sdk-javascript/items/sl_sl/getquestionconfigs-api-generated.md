## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | number | Ne |  |

## Odgovor

Vrne: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigs200Response.ts)

## Primer

[inline-code-attrs-start title = 'getQuestionConfigs Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4c9f2b";
const responseWithoutSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
const skip: number = 50;
const responseWithSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, skip);
[inline-code-end]

---