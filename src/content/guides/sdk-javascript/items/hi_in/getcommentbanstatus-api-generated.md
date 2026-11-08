## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getCommentBanStatus उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_987654";
  const ssoToken: string = "sso_user_abc";

  const statusWithSso: GetCommentBanStatusResponse = await getCommentBanStatus(tenantId, commentId, ssoToken);
  const statusWithoutSso: GetCommentBanStatusResponse = await getCommentBanStatus(tenantId, commentId);
})();
[inline-code-end]