## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| dir | number | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNamesSuccessResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getCommentVoteUserNames उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = 'tenant-9876';
  const commentId: string = 'comment-abc123';
  const dir: number = 1;
  const ssoToken: string = 'sso-xyz789';

  const resultWithSso: GetCommentVoteUserNamesSuccessResponse = await getCommentVoteUserNames(tenantId, commentId, dir, ssoToken);
  const resultWithoutSso: GetCommentVoteUserNamesSuccessResponse = await getCommentVoteUserNames(tenantId, commentId, dir);
}
[inline-code-end]