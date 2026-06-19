## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vrača: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b7c2a';
const configId: string = 'questioncfg_4d3e2a1b';

const response: GetQuestionConfigResponse = await getQuestionConfig(tenantId, configId);

const questionConfig: QuestionConfig | undefined = response.questionConfig;
const customOptions: QuestionConfigCustomOptionsInner[] | undefined = questionConfig?.customOptions;
[inline-code-end]

---