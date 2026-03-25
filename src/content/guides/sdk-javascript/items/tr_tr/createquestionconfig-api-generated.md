## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Evet |  |

## Yanıt

Döndürür: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfig200Response.ts)

## Örnek

[inline-code-attrs-start title = 'createQuestionConfig Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b2c";
const option: QuestionConfigCustomOptionsInner = { id: "opt_yes", label: "Yes, helpful", value: "yes" };
const createQuestionConfigBody: CreateQuestionConfigBody = {
  title: "Article usefulness",
  prompt: "Was this article helpful?",
  type: "singleChoice",
  required: false, // isteğe bağlı parametre gösterildi
  options: [option],
  saveBehavior: "immediate"
};
const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]

---