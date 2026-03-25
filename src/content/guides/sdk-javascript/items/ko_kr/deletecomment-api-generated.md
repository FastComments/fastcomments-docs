## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| contextUserId | string | 아니오 |  |
| isLive | boolean | 아니오 |  |

## 응답

반환값: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteComment200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b3f2a';
const commentId: string = 'cmt_8a1f4d2b';
const contextUserId: string = 'user_102';
const isLive: boolean = true;
const result: DeleteComment200Response = await deleteComment(tenantId, commentId, contextUserId, isLive);
[inline-code-end]

---