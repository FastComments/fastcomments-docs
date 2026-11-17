## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Response

Geeft terug: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getQuestionConfig Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const questionId: string = 'poll-question-789';

const response: GetQuestionConfigResponse = await getQuestionConfig(tenantId, questionId);

const status: APIStatus = response.status;
const config: QuestionConfig = response.config;

const type: string = config.type;
const customOptions: QuestionConfigCustomOptionsInner[] = config.customOptions || [];

if (customOptions.length > 0) {
  const firstOptionLabel: string | undefined = customOptions[0].label;
  console.log(`Config ${config.id} for tenant ${tenantId}, first option: ${firstOptionLabel}`);
}
[inline-code-end]