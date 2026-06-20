## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| commentId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersFromCommentResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getBanUsersFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = "cmt_7f4d2a9b6c";
  const ssoToken: string = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signature";
  const bannedWithoutSso: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(commentId);
  const bannedWithSso: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(commentId, ssoToken);
  console.log(bannedWithoutSso, bannedWithSso);
})();
[inline-code-end]

---