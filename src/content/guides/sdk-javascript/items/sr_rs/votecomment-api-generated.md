## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| urlId | string | Да |  |
| broadcastId | string | Да |  |
| voteBodyParams | VoteBodyParams | Да |  |
| sessionId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Пример

[inline-code-attrs-start title = 'voteComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runVote() {
  const tenantId: string = 'tenant_001';
  const commentId: string = 'cmt_987654';
  const urlId: string = 'url_12345';
  const broadcastId: string = 'brd_56789';
  const voteBody: VoteBodyParams = { direction: 'up' };
  const sessionId: string = 'sess_abc123';
  const sso: string = 'sso_token_xyz';
  const response: VoteResponse = await voteComment(
    tenantId,
    commentId,
    urlId,
    broadcastId,
    voteBody,
    sessionId,
    sso
  );
  console.log(response);
}
runVote();
[inline-code-end]

---