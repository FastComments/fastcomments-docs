---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| broadcastId | string | Ναι |  |
| commentData | CommentData | Ναι |  |
| sessionId | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateCommentPublic200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9a1b2c';
const urlId: string = 'https://www.news-site.com/article/67890';
const broadcastId: string = 'broadcast_2026-06-15-01';
const sessionId: string | undefined = 'sess_abc123xyz';
const sso: string | undefined = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoiam9yZGFuIiwiaWF0IjoxNjI0MDAwMDB9.signature';
const commentData: CommentData = {
  content: 'Insightful piece — I appreciated the data-backed points and sources cited.',
  authorDisplayName: 'Jordan Miles'
} as CommentData;
const result: CreateCommentPublic200Response = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
[inline-code-end]

---