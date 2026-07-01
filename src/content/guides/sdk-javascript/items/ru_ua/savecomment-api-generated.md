## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createCommentParams | CreateCommentParams | Так |  |
| isLive | boolean | Ні |  |
| doSpamCheck | boolean | Ні |  |
| sendEmails | boolean | Ні |  |
| populateNotifications | boolean | Ні |  |

## Відповідь

Повертає: [`SaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад saveComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function submitComment() {
  const tenantId: string = "tenant_9f8e7d6c";
  const commentParams: CreateCommentParams = {
    text: "Great post, thanks for sharing!",
    authorId: "user_123abc",
    mentions: [] as CommentUserMentionInfo[],
    hashtags: [] as CommentUserHashTagInfo[]
  };
  const response: SaveCommentResponse = await saveComment(
    tenantId,
    commentParams,
    true,   // живий (isLive)
    false   // без перевірки спаму (doSpamCheck)
  );
  console.log(response);
}
submitComment();
[inline-code-end]