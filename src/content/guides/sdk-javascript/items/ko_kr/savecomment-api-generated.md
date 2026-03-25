## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createCommentParams | CreateCommentParams | 예 |  |
| isLive | boolean | 아니요 |  |
| doSpamCheck | boolean | 아니요 |  |
| sendEmails | boolean | 아니요 |  |
| populateNotifications | boolean | 아니요 |  |

## 응답

반환값: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveComment200Response.ts)

## 예제

[inline-code-attrs-start title = 'saveComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const createCommentParams: CreateCommentParams = {
  content: 'Great article — helped me fix a production issue in minutes.',
  url: 'https://app.acme.com/blog/performance-tips',
  author: { name: 'Maya Chen', email: 'maya.chen@acme.com' },
  metadata: { locale: 'en-US', appVersion: '4.2.1' }
} as CreateCommentParams;
const isLive: boolean = true;
const doSpamCheck: boolean = true;
const sendEmails: boolean = false;
const populateNotifications: boolean = true;
const result: SaveComment200Response = await saveComment(tenantId, createCommentParams, isLive, doSpamCheck, sendEmails, populateNotifications);
[inline-code-end]

---