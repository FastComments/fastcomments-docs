## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPIGetCommentTextResponse.ts)

## 範例

[inline-code-attrs-start title = 'getCommentText 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_prod_01';
const commentId: string = 'cmt_5f2d9b8a-3e7c-4a1b';
const editKey: string = 'edit_8b3f6c2d4a9e';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const response: PublicAPIGetCommentTextResponse = await getCommentText(tenantId, commentId, editKey, sso);
[inline-code-end]

---