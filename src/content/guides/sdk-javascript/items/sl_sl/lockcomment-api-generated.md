## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Odgovor

Vrne: [`LockCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockCommentResponse.ts)

## Primer

[inline-code-attrs-start title = 'lockComment Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_54321";

  // Z neobveznim SSO žetonom
  const ssoToken: string = "user-abc123";
  const lockedWithSso: LockCommentResponse = await lockComment(tenantId, commentId, broadcastId, ssoToken);

  // Brez SSO žetona
  const lockedWithoutSso: LockCommentResponse = await lockComment(tenantId, commentId, broadcastId);
})();
[inline-code-end]