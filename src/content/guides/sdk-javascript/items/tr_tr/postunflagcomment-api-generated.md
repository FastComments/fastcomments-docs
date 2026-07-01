## Parameters

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| broadcastId | string | Hayır |  |
| tenantId | string | Hayır |  |
| sso | string | Hayır |  |

## Response

Döndürür: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Example

[inline-code-attrs-start title = 'postUnFlagComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // commentId
    "brd_67890",          // broadcastId (isteğe bağlı)
    "tenant_abc",         // tenantId (isteğe bağlı)
    "sso_user_token_789"  // sso (isteğe bağlı)
  );
};
[inline-code-end]