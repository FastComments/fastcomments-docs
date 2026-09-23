## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Yes |  |

## 回應

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateQuestionResult 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1f5e8b2-9a4d-4f3a-8d2e-6b7c9d0e1f2a";
const questionId: string = "qstn_1234567890";

const updateBody: UpdateQuestionResultBody = {
  score: 85,
  comment: "Adjusted based on new criteria",
  meta: [
    { key: "reviewer", value: "john.doe@example.com" },
    { key: "timestamp", value: new Date().toISOString() }
  ]
};

const response: APIEmptyResponse = await updateQuestionResult(tenantId, questionId, updateBody);
[inline-code-end]