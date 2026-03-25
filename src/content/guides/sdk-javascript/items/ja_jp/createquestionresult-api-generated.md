## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createQuestionResultBody | CreateQuestionResultBody | はい |  |

## レスポンス

返却値: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## 例

[inline-code-attrs-start title = 'createQuestionResult の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-72b1f4";
const meta: MetaItem[] = [{ key: "platform", value: "web" }];
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "question-83472",
  commenterId: "user-5521",
  answers: [{ subQuestionId: "sq-1", value: "Yes" }],
  meta, // 任意のメタデータ
  note: "Follow-up requested" // 任意のパラメーターの例
};
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---