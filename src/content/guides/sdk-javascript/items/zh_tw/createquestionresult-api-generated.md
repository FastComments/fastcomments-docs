## 參數

| 名稱 | 型別 | 必要 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createQuestionResultBody | CreateQuestionResultBody | 是 |  |

## 回應

回傳：[`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## 範例

[inline-code-attrs-start title = 'createQuestionResult 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fc_tenant_7a3c_us-east-1";
const metaItem: MetaItem = { key: "referrer", value: "/blog/how-to-comment" };
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "q_42",
  commenterId: "user_1984",
  answer: "yes",
  score: 4,
  meta: [metaItem] // 示範可選的元資料
} as CreateQuestionResultBody;
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---