## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳:[`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## 範例

[inline-code-attrs-start title = 'getQuestionConfig 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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