## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createQuestionResultBody | CreateQuestionResultBody | Evet |  |

## Yanıt

Döndürür: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## Örnek

[inline-code-attrs-start title = 'createQuestionResult Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fc_tenant_7a3c_us-east-1";
const metaItem: MetaItem = { key: "referrer", value: "/blog/how-to-comment" };
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "q_42",
  commenterId: "user_1984",
  answer: "yes",
  score: 4,
  meta: [metaItem] // isteğe bağlı meta veri gösterimi
} as CreateQuestionResultBody;
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---