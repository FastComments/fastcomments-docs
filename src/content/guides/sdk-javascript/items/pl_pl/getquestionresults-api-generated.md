---
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

Zwraca: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'getQuestionResults Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchResults() {
  const tenantId: string = 'c1a2b3d4-5678-90ab-cdef-1234567890ab';
  const urlId: string = 'blog-post-789';
  const startDate: string = '2023-01-01T00:00:00Z';
  const questionIds: string = 'q-abc123,q-def456';
  const response: GetQuestionResultsResponse = await getQuestionResults(
    tenantId,
    urlId,
    undefined,
    startDate,
    undefined,
    questionIds
  );
  console.log(response);
}
[inline-code-end]

---