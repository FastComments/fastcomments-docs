## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| direction | CreateVoteDirectionEnum | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Відповідь

Повертає: [`CreateVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateVoteResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад createVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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