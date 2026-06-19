## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳：[`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigResponse.ts)

## 範例

[inline-code-attrs-start title = 'getQuestionConfig 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b7c2a';
const configId: string = 'questioncfg_4d3e2a1b';

const response: GetQuestionConfigResponse = await getQuestionConfig(tenantId, configId);

const questionConfig: QuestionConfig | undefined = response.questionConfig;
const customOptions: QuestionConfigCustomOptionsInner[] | undefined = questionConfig?.customOptions;
[inline-code-end]

---