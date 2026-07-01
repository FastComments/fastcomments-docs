## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|------|
| commentId | string | Tak |  |
| banEmail | boolean | Nie |  |
| banEmailDomain | boolean | Nie |  |
| banIP | boolean | Nie |  |
| deleteAllUsersComments | boolean | Nie |  |
| bannedUntil | string | Nie |  |
| isShadowBan | boolean | Nie |  |
| updateId | string | Nie |  |
| banReason | string | Nie |  |
| tenantId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`PostBanUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostBanUserFromCommentResponse.ts)

## Przykład

[inline-code-attrs-start title = 'postBanUserFromComment Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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