## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| broadcastId | string | हाँ |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | हाँ |  |
| editKey | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPISetCommentTextResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'setCommentText उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_54321";
  const commentTextUpdateRequest: CommentTextUpdateRequest = {
    text: "Revised comment text with additional details."
  };
  const editKey: string = "edit_abc123";
  const sso: string = "sso_token_456def";

  const response: PublicAPISetCommentTextResponse = await setCommentText(
    tenantId,
    commentId,
    broadcastId,
    commentTextUpdateRequest,
    editKey,
    sso
  );
})();
[inline-code-end]