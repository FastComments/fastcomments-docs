## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIChildCommentsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getCommentChildren उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchChildren() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const ssoToken: string = "sso_user_abc123";

  const responseWithSSO: ModerationAPIChildCommentsResponse = await getCommentChildren(tenantId, commentId, ssoToken);
  const responseWithoutSSO: ModerationAPIChildCommentsResponse = await getCommentChildren(tenantId, commentId);
}

fetchChildren();
[inline-code-end]