## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`BlockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockSuccess.ts)

## उदाहरण

[inline-code-attrs-start title = 'blockFromCommentPublic उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoBlock() {
  const tenantId: string = "tenant_9f8b7c";
  const commentId: string = "comment_3e2d1a";
  const blockParams: PublicBlockFromCommentParams = {
    reason: "harassment",
    expiresInHours: 48
  };
  const ssoToken: string = "sso_5g6h7i";

  const resultWithSso: BlockSuccess = await blockFromCommentPublic(tenantId, commentId, blockParams, ssoToken);
  const resultWithoutSso: BlockSuccess = await blockFromCommentPublic(tenantId, commentId, blockParams);
}
[inline-code-end]