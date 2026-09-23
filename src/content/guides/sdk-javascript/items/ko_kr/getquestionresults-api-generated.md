## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 아니오 |  |
| userId | string | 아니오 |  |
| startDate | string | 아니오 |  |
| questionId | string | 아니오 |  |
| questionIds | string | 아니오 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## 예시

[inline-code-attrs-start title = 'getQuestionResults 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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