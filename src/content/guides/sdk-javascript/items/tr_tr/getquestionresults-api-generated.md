## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Hayır |  |
| userId | string | Hayır |  |
| startDate | string | Hayır |  |
| questionId | string | Hayır |  |
| questionIds | string | Hayır |  |
| skip | number | Hayır |  |

## Yanıt

Döndürür: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getQuestionResults Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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