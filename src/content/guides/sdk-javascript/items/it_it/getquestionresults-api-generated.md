## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | No |  |
| userId | string | No |  |
| startDate | string | No |  |
| questionId | string | No |  |
| questionIds | string | No |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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