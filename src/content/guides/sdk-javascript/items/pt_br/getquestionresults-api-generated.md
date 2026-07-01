---
## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Não |  |
| userId | string | Não |  |
| startDate | string | Não |  |
| questionId | string | Não |  |
| questionIds | string | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetQuestionResultsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse1.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const urlId: string = "post-456";
  const userId: string = "user-789";
  const startDate: string = "2023-09-01T00:00:00Z";
  const questionId: string = "q-101";
  const questionIds: string = "q-102,q-103";
  const skip: number = 20;

  const results: GetQuestionResultsResponse1 = await getQuestionResults(
    tenantId,
    urlId,
    userId,
    startDate,
    questionId,
    questionIds,
    skip
  );

  console.log(results);
})();
[inline-code-end]

---