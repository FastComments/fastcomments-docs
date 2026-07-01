## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`PinCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinCommentResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'pinComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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