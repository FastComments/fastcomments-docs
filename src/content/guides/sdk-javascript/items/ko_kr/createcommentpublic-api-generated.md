## 매개변수

| 이름 | 타입 | 필수 여부 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| broadcastId | string | 예 |  |
| commentData | CommentData | 예 |  |
| sessionId | string | 아니요 |  |
| sso | string | 아니요 |  |
| locale | string | 아니요 |  |

## 응답

반환값: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentsResponseWithPresence.ts)

## 예제

[inline-code-attrs-start title = 'createCommentPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a 团a4b2c3d4e5f6a7b8c9d0e1f2';
const urlId: string = 'product-launch-post';
const broadcastId: string = 'fc-live-2024-keynote';

const commentData: CommentData = {
  comment: 'Great article! Really looking forward to the upcoming release.',
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