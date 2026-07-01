## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| commentId | string | Ja |  |
| banEmail | boolean | Nein |  |
| banEmailDomain | boolean | Nein |  |
| banIP | boolean | Nein |  |
| deleteAllUsersComments | boolean | Nein |  |
| bannedUntil | string | Nein |  |
| isShadowBan | boolean | Nein |  |
| updateId | string | Nein |  |
| banReason | string | Nein |  |
| tenantId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`PostBanUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostBanUserFromCommentResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'postBanUserFromComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runBan() {
  const commentId: string = "cmt_5f8a2b3c";
  const banEmail: boolean = true;
  const banIP: boolean = false;
  const deleteAllUsersComments: boolean = true;
  const bannedUntil: string = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString();
  const isShadowBan: boolean = false;
  const banReason: string = "Repeated spam posting";
  const tenantId: string = "tenant_12345";

  const response: PostBanUserFromCommentResponse = await postBanUserFromComment(
    commentId,
    banEmail,
    undefined,
    banIP,
    deleteAllUsersComments,
    bannedUntil,
    isShadowBan,
    undefined,
    banReason,
    tenantId
  );
  console.log(response);
}
runBan();
[inline-code-end]