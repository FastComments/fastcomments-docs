## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'pinComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoPinComment() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_54321";

  const resultWithoutSso: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId);
  const ssoToken: string = "sso_abcde12345";
  const resultWithSso: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId, ssoToken);
}

demoPinComment();
[inline-code-end]