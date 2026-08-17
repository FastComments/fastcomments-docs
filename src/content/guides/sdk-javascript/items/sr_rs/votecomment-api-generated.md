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

Враћа: [`VoteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример voteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_9f8e7d6c";
const urlId: string = "url_123456";
const broadcastId: string = "bcast_2024_01";

const voteBodyParams: VoteBodyParams = {
  vote: "up",               // нпр., "up" | "down"
  weight: 1,                // опционо што одређује тежину гласа
};

const sessionId: string = "sess_abc123def";
const sso: string = "sso_token_xyz";

const result: VoteCommentResponse = await voteComment(
  tenantId,
  commentId,
  urlId,
  broadcastId,
  voteBodyParams,
  sessionId,
  sso
);
[inline-code-end]