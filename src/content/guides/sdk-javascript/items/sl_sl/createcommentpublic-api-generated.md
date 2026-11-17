## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| broadcastId | string | Da |  |
| commentData | CommentData | Da |  |
| sessionId | string | Ne |  |
| sso | string | Ne |  |
| locale | string | Ne |  |

## Odgovor

Vrne: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentsResponseWithPresence.ts)

## Primer

[inline-code-attrs-start title = 'Primer za createCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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