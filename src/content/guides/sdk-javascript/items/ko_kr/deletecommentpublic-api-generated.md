---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 예 |  |
| editKey | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPIDeleteCommentResponse.ts)

## 예제

[inline-code-attrs-start title = 'deleteCommentPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a3b2c";
const commentId: string = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
const broadcastId: string = "site:homepage";
const editKey: string | undefined = "edk_9f8b7c6";
const sso: string | undefined = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dXNlcjoxMjM.zXlKaGJHY2lPaUpJVXpJMU5pSXNJbXRwWkNJNkltRnpaWEp0YjI1bFpTSTZJbU5zYjJOcGRHbHZiaUk2SW1Wa1pYTmpaWE5vWldGelpTMWZkR2x2TG05eVpXTm9iM0pwYjI0";
const result: PublicAPIDeleteCommentResponse = await deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso);
[inline-code-end]

---