## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Не |  |
| userId | string | Не |  |
| startDate | string | Не |  |
| questionId | string | Не |  |
| questionIds | string | Не |  |
| skip | number | Не |  |

## Отговор

Връща: [`GetQuestionResultsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse1.ts)

## Пример

[inline-code-attrs-start title = 'getQuestionResults Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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