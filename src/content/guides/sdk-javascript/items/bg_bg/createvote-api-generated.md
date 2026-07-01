## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| direction | CreateVoteDirectionEnum | Да |  |
| userId | string | Не |  |
| anonUserId | string | Не |  |

## Отговор

Връща: [`CreateVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateVoteResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "comment_987654";
const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.Upvote;
const userId: string = "user_7f9a2b";

const voteResult: CreateVoteResponse = await createVote(
  tenantId,
  commentId,
  direction,
  userId
);
[inline-code-end]