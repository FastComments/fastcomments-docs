## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Ja |  |
| editKey | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPISetCommentTextResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'setCommentText Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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