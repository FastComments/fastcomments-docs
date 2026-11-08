## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| includeEmail | boolean | नहीं |  |
| includeIP | boolean | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getModerationComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // केवल आवश्यक पैरामीटर के साथ कॉल करें
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // वैकल्पिक पैरामीटर के साथ कॉल करें
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const sso: string = "sso-token-abc123";
  const detailedResponse: ModerationAPICommentResponse = await getModerationComment(
    tenantId,
    commentId,
    includeEmail,
    includeIP,
    sso
  );
}
[inline-code-end]