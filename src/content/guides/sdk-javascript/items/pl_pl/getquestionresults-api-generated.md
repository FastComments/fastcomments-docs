## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| urlId | string | Nie |  |
| userId | string | Nie |  |
| startDate | string | Nie |  |
| questionId | string | Nie |  |
| questionIds | string | Nie |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetQuestionResultsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse1.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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