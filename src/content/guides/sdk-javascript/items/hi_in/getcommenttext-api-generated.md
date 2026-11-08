## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| editKey | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPIGetCommentTextResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getCommentText उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const commentId: string = "comment-987654321";
const editKey: string = "edit-key-abc123";
const sso: string = "sso-token-xyz789";

const commentWithEdit: PublicAPIGetCommentTextResponse = await getCommentText(
  tenantId,
  commentId,
  editKey,
  sso
);

const commentBasic: PublicAPIGetCommentTextResponse = await getCommentText(
  tenantId,
  commentId
);
[inline-code-end]