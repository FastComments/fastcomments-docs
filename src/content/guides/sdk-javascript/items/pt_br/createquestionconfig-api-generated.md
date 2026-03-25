## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Sim |  |

## Resposta

Retorna: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfig200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b2c";
const option: QuestionConfigCustomOptionsInner = { id: "opt_yes", label: "Yes, helpful", value: "yes" };
const createQuestionConfigBody: CreateQuestionConfigBody = {
  title: "Article usefulness",
  prompt: "Was this article helpful?",
  type: "singleChoice",
  required: false, // parâmetro opcional demonstrado
  options: [option],
  saveBehavior: "immediate"
};
const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]

---