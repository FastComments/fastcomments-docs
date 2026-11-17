## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| broadcastId | string | Так |  |
| commentData | CommentData | Так |  |
| sessionId | string | Ні |  |
| sso | string | Ні |  |
| locale | string | Ні |  |

## Відповідь

Повертає: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentsResponseWithPresence.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад createCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a 团a4b2c3d4e5f6a7b8c9d0e1f2';
const urlId: string = 'product-launch-post';
const broadcastId: string = 'fc-live-2024-keynote';

const commentData: CommentData = {
  comment: 'Чудова стаття! Дуже чекаю на майбутній реліз.',
  email: 'jane.doe@example.com',
  name: 'Jane Doe',
  url: 'https://example.com/blog/product-launch-post'
};

const sessionId: string = 'sess_8f14e45fceea167a5a36dedd4bea2543';
const locale: string = 'en';

const response: SaveCommentsResponseWithPresence = await createCommentPublic(
  tenantId,
  urlId,
  broadcastId,
  commentData,
  sessionId,
  undefined,
  locale
);

const status: APIStatus = response.status;
[inline-code-end]