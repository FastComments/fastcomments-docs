## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Não |  |
| userId | string | Não |  |
| startDate | string | Não |  |
| questionId | string | Não |  |
| questionIds | string | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9b3f";
  const urlId: string = "survey-2026-spring";
  const userId: string = "user_00123";
  const startDate: string = "2026-04-01T00:00:00Z";
  const questionIds: string = "q_42,q_43";
  const skip: number = 0;
  const result: GetQuestionResults200Response = await getQuestionResults(tenantId, urlId, userId, startDate, undefined, questionIds, skip);
  console.log(result);
})();
[inline-code-end]

---