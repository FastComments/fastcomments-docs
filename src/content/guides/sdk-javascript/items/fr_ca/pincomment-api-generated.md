## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`PinCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de pinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant-001';
  const commentId: string = 'comment-5678';
  const broadcastId: string = 'broadcast-2023';
  const ssoToken: string = 'sso-xyz-789';

  const pinResult: PinCommentResponse = await pinComment(tenantId, commentId, broadcastId);
  const pinResultWithSso: PinCommentResponse = await pinComment(tenantId, commentId, broadcastId, ssoToken);
})();
[inline-code-end]