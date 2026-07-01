## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| dir | number | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentVoteUserNamesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNamesResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentVoteUserNames'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetCommentVoteUserNames() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_5f2a1e3b";
  const dir: number = 1; // rosnąco

  const votesWithoutSSO: GetCommentVoteUserNamesResponse = await getCommentVoteUserNames(
    tenantId,
    commentId,
    dir
  );

  const ssoToken: string = "sso_abcdef123456";
  const votesWithSSO: GetCommentVoteUserNamesResponse = await getCommentVoteUserNames(
    tenantId,
    commentId,
    dir,
    ssoToken
  );

  console.log(votesWithoutSSO, votesWithSSO);
}
[inline-code-end]

---