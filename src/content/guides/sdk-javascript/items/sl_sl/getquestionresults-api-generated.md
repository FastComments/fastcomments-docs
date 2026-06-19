## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Ne |  |
| userId | string | Ne |  |
| startDate | string | Ne |  |
| questionId | string | Ne |  |
| questionIds | string | Ne |  |
| skip | number | Ne |  |

## Odgovor

Vrača: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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