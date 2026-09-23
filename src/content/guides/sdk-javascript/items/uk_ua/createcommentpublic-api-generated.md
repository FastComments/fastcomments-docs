## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| broadcastId | string | Так |  |
| commentData | CommentData | Так |  |
| sessionId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentsResponseWithPresence.ts)

## Приклад

[inline-code-attrs-start title = 'createCommentPublic Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant-456";
  const urlId: string = "url-789";
  const broadcastId: string = "broadcast-101112";
  const commentData: CommentData = { text: "This is a comment", userId: "user-123" };
  const sessionId: string = "session-131415";
  const sso: string = "sso-token-xyz";
  const result: SaveCommentsResponseWithPresence = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
  const resultNoOpt: SaveCommentsResponseWithPresence = await createCommentPublic(tenantId, urlId, broadcastId, commentData);
}
[inline-code-end]