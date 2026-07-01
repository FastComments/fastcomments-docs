## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createCommentParams | CreateCommentParams | Да |  |
| isLive | boolean | Нет |  |
| doSpamCheck | boolean | Нет |  |
| sendEmails | boolean | Нет |  |
| populateNotifications | boolean | Нет |  |

## Response

Возвращает: [`SaveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример saveComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    true,   // isLive
    false   // doSpamCheck
  );
  console.log(response);
}
submitComment();
[inline-code-end]