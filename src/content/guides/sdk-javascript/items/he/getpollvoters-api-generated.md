מי הצביע לאיזו אפשרות. סקרים אנונימיים לעולם לא מחזירים תשובה, וסקרים הפתוחים למנהלים בלבד מחזירים תשובה רק עבור מנהל או מנחה.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| urlId | string | No |  |
| page | number | No |  |
| search | string | No |  |
| sso | string | No |  |

## Response

מחזיר: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPollVoterNamesSuccessResponse.ts)

## Example

[inline-code-attrs-start title = 'דוגמה ל-getPollVoters'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'a1b2c3d4e5f6';

const result: GetPollVoterNamesSuccessResponse = await getPollVoters(
  tenantId,
  'comment_123456',
  'https://example.com/blog/my-post',
  1,
  'jane.doe',
  'SSO_TOKEN_98765'
);

const status: APIStatus = result.status;
const voters: PollVoterEntry[] = result.voters;

console.log(`Total voters on page: ${voters.length}`);
[inline-code-end]