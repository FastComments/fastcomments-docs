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
const tenantId: string = 'tenant_acme_922';
const mention: CommentUserMentionInfo = { userId: 'u-204', displayName: 'Maria Gomez', username: 'maria.g' };
const hashtag: CommentUserHashTagInfo = { tag: 'frontend', indices: [45, 53] };
const createCommentParams: Array<CreateCommentParams> = [
  {
    content: 'I replaced the legacy DOM manipulation with a small component — performance improved by ~30%.',
    authorName: 'Samuel Park',
    authorEmail: 'samuel.park@devteam.io',
    permalink: '/guides/performance-tuning',
    createdAt: '2026-01-11T09:15:00Z',
    userMentions: [mention],
    userHashtags: [hashtag]
  }
];
const responses: Array<SaveComment200Response> = await saveCommentsBulk(tenantId, createCommentParams, true, true, false, true);
[inline-code-end]
