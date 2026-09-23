הקולות האישיים מאחורי ספירת הצבעה של סקר אחד, מהישן ביותר לחדש ביותר.

סקר שייך לתגובה, ולכן הקולות תמיד נקראים סקר אחד בכל פעם - נדרש **commentId**. זה שומר שכל שאילתה תתבצע על האינדקסים שכבר קיימים באוסף.

מציית לפרטיות של הסקר: קולות של סקר אנונימי אינם ניתנים לקריאה (**poll-anonymous**), כאן או לפי מזהה.

## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voterId | string | No |  |
| optionId | string | No |  |
| skip | number | No |  |

## Response

Returns: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVotesResponse.ts)

## Example

[inline-code-attrs-start title = 'דוגמת getPollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const voterId: string = "user_abc";
const optionId: string = "opt_1";
const skip: number = 20;

const pollResult: GetPollVotesResponse = await getPollVotes(
  tenantId,
  commentId,
  voterId,
  optionId,
  skip
);

console.log(pollResult);
[inline-code-end]

---