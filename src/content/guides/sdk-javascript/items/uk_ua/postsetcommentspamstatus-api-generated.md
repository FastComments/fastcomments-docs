## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| spam | boolean | No |  |
| permNotSpam | boolean | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад postSetCommentSpamStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const commentId: string = "comment_1001";

const spam: boolean = true;
const permNotSpam: boolean = false;
const broadcastId: string = "broadcast_2023";
const sso: string = "sso_user_5678";

const resultFull: APIEmptyResponse = await postSetCommentSpamStatus(
  tenantId,
  commentId,
  spam,
  permNotSpam,
  broadcastId,
  sso
);

const resultMinimal: APIEmptyResponse = await postSetCommentSpamStatus(
  tenantId,
  commentId
);
[inline-code-end]