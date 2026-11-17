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
declare const getQuestionResults: (
  tenantId: string,
  urlId?: string,
  userId?: string,
  startDate?: string,
  questionId?: string,
  questionIds?: string,
  skip?: number
) => Promise<GetQuestionResultsResponse>;

const tenantId: string = 'abc123def456';
const urlId: string = 'https://example.com/blog/my-post';
const userId: string = 'user_98765';
const startDate: string = '2024-01-01T00:00:00.000Z';
const skip: number = 0;

const results: GetQuestionResultsResponse = await getQuestionResults(
  tenantId,
  urlId,
  userId,
  startDate,
  undefined,
  'question_1,question_2',
  skip
);

const firstResult: QuestionResult | undefined = results[0];
[inline-code-end]