## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigs200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_9876";
const configsWithoutSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
const configsWithSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, 20);
[inline-code-end]

---