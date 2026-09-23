## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Returnerer: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getQuestionConfig Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const questionId: string = "q-987654321";

const configResponse: GetQuestionConfigResponse = await getQuestionConfig(tenantId, questionId);

const status: APIStatus = configResponse.status;
const questionConfig?: QuestionConfig = configResponse.question;
const firstOption?: QuestionConfigCustomOptionsInner = questionConfig?.customOptions?.[0];
[inline-code-end]