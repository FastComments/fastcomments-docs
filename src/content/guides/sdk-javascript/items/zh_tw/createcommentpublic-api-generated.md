## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 是 |  |
| commentData | CommentData | 是 |  |
| sessionId | string | 否 |  |
| sso | string | 否 |  |
| locale | string | 否 |  |

## 回應

回傳:[`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentsResponseWithPresence.ts)

## 範例

[inline-code-attrs-start title = 'createCommentPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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