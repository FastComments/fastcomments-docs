## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`UnBlockCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublicResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'unBlockCommentPublic Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const commentId: string = "cmt-20230915-001";
const unblockParams: PublicBlockFromCommentParams = {
  reason: "User appealed and was cleared",
  unblockExpiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString()
};
const ssoToken: string = "sso-3d9f8a7b";

const result: UnBlockCommentPublicResponse = await unBlockCommentPublic(
  tenantId,
  commentId,
  unblockParams,
  ssoToken
);
[inline-code-end]