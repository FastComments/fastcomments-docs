## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| feedPost | FeedPost | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateFeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-global-tenant-42';
const id: string = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';

const asset: FeedPostMediaItemAsset = {
  url: 'https://cdn.acme.com/images/product-launch.jpg',
  mimeType: 'image/jpeg',
  width: 1200,
  height: 630
};

const mediaItem: FeedPostMediaItem = {
  id: 'media-001',
  type: 'image',
  asset
};

const link: FeedPostLink = {
  url: 'https://acme.com/blog/product-launch',
  title: 'Product Launch Details'
};

const feedPost: FeedPost = {
  title: 'Introducing the Q3 Product Suite',
  body: 'We are excited to unveil our new lineup for Q3, focusing on performance and security improvements.',
  media: [mediaItem],     // כולל מערך אופציונלי
  links: [link],          // כולל קישורים אופציונליים
  isPublished: true       // דגל פרסום אופציונלי משמש כאן
};

const result: FlagCommentPublic200Response = await updateFeedPost(tenantId, id, feedPost);
[inline-code-end]

---