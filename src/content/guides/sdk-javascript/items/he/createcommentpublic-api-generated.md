## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| broadcastId | string | כן |  |
| commentData | CommentData | כן |  |
| sessionId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת createCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_42';
const urlId: string = 'article-2026-03-25-tech-deep-dive';
const broadcastId: string = 'live-broadcast-001';
const sessionId: string = 'sess_9f8e7d6a3b';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const commentData: CommentData = {
  content: 'Great reporting — appreciated the depth on performance tradeoffs.',
  authorName: 'Jordan M.',
  language: 'en-US',
  metadata: { client: 'web' }
};
const result: CreateCommentPublic200Response = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
[inline-code-end]

---