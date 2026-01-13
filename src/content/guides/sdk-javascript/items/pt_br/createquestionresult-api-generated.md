---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| createQuestionResultBody | CreateQuestionResultBody | Sim |  |

## Resposta

Retorna: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fc_tenant_7a3c_us-east-1";
const metaItem: MetaItem = { key: "referrer", value: "/blog/how-to-comment" };
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "q_42",
  commenterId: "user_1984",
  answer: "yes",
  score: 4,
  meta: [metaItem] // metadados opcionais demonstrados
} as CreateQuestionResultBody;
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---