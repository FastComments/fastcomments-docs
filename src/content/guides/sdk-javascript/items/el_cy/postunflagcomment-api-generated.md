## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|------------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Απάντηση

Επιστρέφει: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postUnFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // commentId
    "brd_67890",          // broadcastId (προαιρετικό)
    "tenant_abc",         // tenantId (προαιρετικό)
    "sso_user_token_789"  // sso (προαιρετικό)
  );
};
[inline-code-end]