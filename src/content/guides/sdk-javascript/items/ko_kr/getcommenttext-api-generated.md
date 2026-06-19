## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| editKey | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPIGetCommentTextResponse.ts)

## 예제

[inline-code-attrs-start title = 'getCommentText 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_prod_01';
const commentId: string = 'cmt_5f2d9b8a-3e7c-4a1b';
const editKey: string = 'edit_8b3f6c2d4a9e';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const response: PublicAPIGetCommentTextResponse = await getCommentText(tenantId, commentId, editKey, sso);
[inline-code-end]

---