## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'unBlockCommentPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42e8a1';
const commentId: string = 'cmt_9b3f2d';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: 'abusive_language',
  blockedByModeratorId: 'mod_17',
  note: 'Targeted harassment; review complete',
  unblockRequestedAt: new Date().toISOString()
};
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const result: UnBlockCommentPublic200Response = await unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]

---