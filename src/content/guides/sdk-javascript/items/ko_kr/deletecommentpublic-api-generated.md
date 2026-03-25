## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 예 |  |
| editKey | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'deleteCommentPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f2c9b';
const commentId: string = 'comment-7c3a9f2d';
const broadcastId: string = 'article-2026-03-20';
const editKey: string | undefined = 'ek_pub_abc12345';
const sso: string | undefined = 'sso_eyJhbGciOiJIUzI1Ni';

const result: DeleteCommentPublic200Response = await deleteCommentPublic(
  tenantId,
  commentId,
  broadcastId,
  editKey,
  sso
);
[inline-code-end]

---