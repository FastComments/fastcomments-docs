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

Retorna: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42f6';
const urlId: string = 'product-page-123';
const userId: string = 'user_789';
const startDate: string = '2024-05-01';
const questionIds: string = 'q-112,q-113';
const skip: number = 20;
const results: GetQuestionResultsResponse = await getQuestionResults(tenantId, urlId, userId, startDate, undefined, questionIds, skip);
[inline-code-end]

---