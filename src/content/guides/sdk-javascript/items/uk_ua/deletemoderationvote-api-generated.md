---
## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| voteId | string | Так |  |
| broadcastId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Приклад

[inline-code-attrs-start title = 'deleteModerationVote Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeleteModerationVote(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6a";
  const commentId: string = "comment_4d3e2f1a";
  const voteId: string = "vote_12345abcde";
  const broadcastId: string | undefined = "broadcast_2023_09_15";
  const sso: string | undefined = "sso_user_7890token";

  const result: VoteDeleteResponse = await deleteModerationVote(
    tenantId,
    commentId,
    voteId,
    broadcastId,
    sso
  );

  console.log(result);
}
[inline-code-end]

---