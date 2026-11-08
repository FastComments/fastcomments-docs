## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | नहीं |  |
| userId | string | नहीं |  |
| startDate | string | नहीं |  |
| questionId | string | नहीं |  |
| questionIds | string | नहीं |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getQuestionResults उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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