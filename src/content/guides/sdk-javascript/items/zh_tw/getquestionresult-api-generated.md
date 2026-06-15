## 參數

| 名稱 | 類型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳：[`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## 範例

[inline-code-attrs-start title = 'getQuestionResult 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a7c9';
const questionId: string = 'q_8d4f1b2c3a';
const options: { includeMeta?: boolean } = { includeMeta: true }; // 可選參數示範
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, questionId);
const apiStatus: APIStatus | undefined = (result as unknown as { apiStatus?: APIStatus }).apiStatus;
const question: QuestionResult | undefined = (result as unknown as { question?: QuestionResult }).question;
[inline-code-end]

---