## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| commentId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentBanStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const commentId: string = 'd3b07384-d9f1-4b3a-9f82-1a2b3c4d5e6f';
  const banStatusDefault: GetCommentBanStatusResponse = await getCommentBanStatus(commentId);
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0MjM0IiwiaWF0IjoxNjMwMDAwMDB9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
  const banStatusWithSso: GetCommentBanStatusResponse = await getCommentBanStatus(commentId, ssoToken);
  console.log(banStatusDefault, banStatusWithSso);
})();
[inline-code-end]

---