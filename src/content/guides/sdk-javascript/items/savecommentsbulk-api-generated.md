## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | Array<CreateCommentParams> | Yes |  |
| isLive | boolean | No |  |
| doSpamCheck | boolean | No |  |
| sendEmails | boolean | No |  |
| populateNotifications | boolean | No |  |

## Response

Returns: `Array<SaveComment200Response`

## Example

[inline-code-attrs-start title = 'saveCommentsBulk Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-8421';
const mentions1: Array<CommentUserMentionInfo> = [{ userId: 'user_552', username: 'dev_john', start: 42, end: 51 }];
const hashtags1: Array<CommentUserHashTagInfo> = [{ tag: 'pagination', start: 55, end: 65 }];
const mentions2: Array<CommentUserMentionInfo> = [{ userId: 'user_7814', username: 'Marissa Chen', start: 12, end: 28 }];
const createCommentParams: Array<CreateCommentParams> = [
  {
    postId: 'blog-2026-01-01-new-api',
    userId: 'user_7814',
    userName: 'Marissa Chen',
    content: 'Great write-up — I especially liked the section on pagination.',
    parentId: null,
    createdAt: '2026-01-09T12:34:00Z',
    mentions: mentions1,
    hashtags: hashtags1
  },
  {
    postId: 'blog-2026-01-01-new-api',
    userId: 'user_9932',
    userName: 'Omar Ruiz',
    content: 'Replying to @Marissa — can you share the dataset?',
    parentId: 'comment_210',
    createdAt: '2026-01-09T12:45:00Z',
    mentions: mentions2,
    hashtags: []
  }
];
const responses: Array<SaveComment200Response> = await saveCommentsBulk(tenantId, createCommentParams, true, true, false, true);
[inline-code-end]
