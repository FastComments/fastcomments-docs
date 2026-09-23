## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Odziv

Vrne: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostRemoveCommentApiResponse.ts)

## Primer

[inline-code-attrs-start title = 'postRemoveComment Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeCommentExample() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_555";
  const sso: string = "sso_token_abc";

  const response: PostRemoveCommentApiResponse = await postRemoveComment(
    tenantId,
    commentId,
    broadcastId,
    sso
  );
  console.log(response);
}

removeCommentExample();
[inline-code-end]