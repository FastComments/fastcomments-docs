## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionResultBody | CreateQuestionResultBody | Yes |  |

## レスポンス

返却: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResultResponse.ts)

## 例

[inline-code-attrs-start title = 'createQuestionResult の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "q_9876",
  answer: "Yes",
  score: 5,
  meta: [
    { key: "source", value: "survey" }
  ]
};

const result: CreateQuestionResultResponse = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---