## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnblockSuccess.ts)

## उदाहरण

[inline-code-attrs-start title = 'unBlockCommentPublic उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c9f1e2a4-5b6d-4f8a-9c2e-123456789abc";
const commentId: string = "d3b2c1a9-8f7e-4d6b-9a0b-987654321def";

const unblockParams: PublicBlockFromCommentParams = {
    reason: "User request resolved",
    moderatorId: "mod-456"
};

const ssoToken: string = "sso-token-789xyz";

const unblockResult: UnblockSuccess = await unBlockCommentPublic(tenantId, commentId, unblockParams, ssoToken);
const unblockResultNoSso: UnblockSuccess = await unBlockCommentPublic(tenantId, commentId, unblockParams);
[inline-code-end]