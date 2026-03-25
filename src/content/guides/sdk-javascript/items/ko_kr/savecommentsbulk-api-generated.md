## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createCommentParams | Array<CreateCommentParams> | 예 |  |
| isLive | boolean | 아니요 |  |
| doSpamCheck | boolean | 아니요 |  |
| sendEmails | boolean | 아니요 |  |
| populateNotifications | boolean | 아니요 |  |

## 응답

반환: `Array<SaveComment200Response`

## 예제

[inline-code-attrs-start title = 'saveCommentsBulk 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-01';
const mentions1: CommentUserMentionInfo[] = [{ userId: 'user-123', displayName: 'Jane Doe' }];
const hashtags1: CommentUserHashTagInfo[] = [{ tag: 'typescript' }];
const createCommentParams: CreateCommentParams[] = [
  {
    content: 'Great insights on async/await patterns.',
    authorName: 'John Smith',
    authorEmail: 'john.smith@acme.com',
    externalId: 'comment-001',
    createdAt: '2026-03-25T10:15:00Z',
    userMentions: mentions1,
    userHashTags: hashtags1
  },
  {
    content: 'I prefer using Promise.all for bulk ops.',
    authorName: 'Emily Turner',
    authorEmail: 'emily.turner@acme.com',
    externalId: 'comment-002',
    createdAt: '2026-03-25T10:20:00Z'
  }
];
const result: SaveComment200Response[] = await saveCommentsBulk(tenantId, createCommentParams, true, true, false, true);
[inline-code-end]

---