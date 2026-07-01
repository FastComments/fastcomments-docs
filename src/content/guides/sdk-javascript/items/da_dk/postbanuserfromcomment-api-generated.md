## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| banEmail | boolean | Nej |  |
| banEmailDomain | boolean | Nej |  |
| banIP | boolean | Nej |  |
| deleteAllUsersComments | boolean | Nej |  |
| bannedUntil | string | Nej |  |
| isShadowBan | boolean | Nej |  |
| updateId | string | Nej |  |
| banReason | string | Nej |  |
| tenantId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`PostBanUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostBanUserFromCommentResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'postBanUserFromComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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