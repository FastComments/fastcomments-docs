## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| dir | number | Yes |  |
| sso | string | No |  |

## Отговор

Връща: [`GetCommentVoteUserNamesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNamesResponse.ts)

## Пример

[inline-code-attrs-start title = 'getCommentVoteUserNames Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetCommentVoteUserNames() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_5f2a1e3b";
  const dir: number = 1; // възходящо

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