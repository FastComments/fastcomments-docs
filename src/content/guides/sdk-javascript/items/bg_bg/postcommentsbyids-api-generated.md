## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| commentsByIdsParams | CommentsByIdsParams | Да |  |
| sso | string | Не |  |

## Отговор

Връща: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIChildCommentsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за postCommentsByIds'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentsByIdsParams: CommentsByIdsParams = {
  ids: ['cmt_7f3b2a', 'cmt_9d1e4c'],
  includeReplies: true,
  limit: 25,
  threadId: 'thread_21a9f0'
};
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOjEyMywiZW1haWwiOiJ1c2VyQGV4YW1wbGUuY29tIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const result: ModerationAPIChildCommentsResponse = await postCommentsByIds(commentsByIdsParams, ssoToken);
[inline-code-end]

---