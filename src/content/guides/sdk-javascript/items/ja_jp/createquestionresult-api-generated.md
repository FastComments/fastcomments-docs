## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createQuestionResultBody | CreateQuestionResultBody | はい |  |

## レスポンス

返却値: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## 例

[inline-code-attrs-start title = 'createQuestionResult の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fc_tenant_7a3c_us-east-1";
const metaItem: MetaItem = { key: "referrer", value: "/blog/how-to-comment" };
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "q_42",
  commenterId: "user_1984",
  answer: "yes",
  score: 4,
  meta: [metaItem] // 任意のメタデータの例示
} as CreateQuestionResultBody;
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---