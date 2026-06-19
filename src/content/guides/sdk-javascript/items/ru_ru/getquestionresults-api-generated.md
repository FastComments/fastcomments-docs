## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Нет |  |
| userId | string | Нет |  |
| startDate | string | Нет |  |
| questionId | string | Нет |  |
| questionIds | string | Нет |  |
| skip | number | Нет |  |

## Ответ

Возвращает: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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