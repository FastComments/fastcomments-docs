## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

Returns: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## 範例

[inline-code-attrs-start title = 'getQuestionConfig 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const questionId: string = "q-987654321";

const configResponse: GetQuestionConfigResponse = await getQuestionConfig(tenantId, questionId);

const status: APIStatus = configResponse.status;
const questionConfig?: QuestionConfig = configResponse.question;
const firstOption?: QuestionConfigCustomOptionsInner = questionConfig?.customOptions?.[0];
[inline-code-end]