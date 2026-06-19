## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createCommentParams | Array<CreateCommentParams> | 예 |  |
| isLive | boolean | 아니오 |  |
| doSpamCheck | boolean | 아니오 |  |
| sendEmails | boolean | 아니오 |  |
| populateNotifications | boolean | 아니오 |  |

## 응답

반환: `Array<SaveCommentsBulkResponse`

## 예제

[inline-code-attrs-start title = 'saveCommentsBulk 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme_corp_tenant_12';
const createCommentParams: Array<CreateCommentParams> = [
  {
    content: 'Really helpful breakdown of the migration steps.',
    threadId: 'thread_2026_08',
    authorName: 'Maya Singh',
    authorEmail: 'maya.singh@startup.io',
    mentions: [{ userId: 'user_314', displayName: 'Leo Park' }],
    hashtags: [{ tag: 'migration' }],
    createdAt: '2026-06-19T12:00:00Z'
  }
];
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const sendEmails: boolean = false;
const populateNotifications: boolean = true;
const responses: Array<SaveCommentsBulkResponse> = await saveCommentsBulk(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]