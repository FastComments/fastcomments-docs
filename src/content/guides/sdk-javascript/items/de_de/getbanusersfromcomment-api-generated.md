## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersFromCommentResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getBanUsersFromComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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