## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`LockCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple lockComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_54321";

  // Avec le token SSO facultatif
  const ssoToken: string = "user-abc123";
  const lockedWithSso: LockCommentResponse = await lockComment(tenantId, commentId, broadcastId, ssoToken);

  // Sans token SSO
  const lockedWithoutSso: LockCommentResponse = await lockComment(tenantId, commentId, broadcastId);
})();
[inline-code-end]

---