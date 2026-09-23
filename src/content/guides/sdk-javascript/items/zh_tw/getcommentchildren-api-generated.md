## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

返回：[`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIChildCommentsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getCommentChildren 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchChildren() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const ssoToken: string = "sso_user_abc123";

  const responseWithSSO: ModerationAPIChildCommentsResponse = await getCommentChildren(tenantId, commentId, ssoToken);
  const responseWithoutSSO: ModerationAPIChildCommentsResponse = await getCommentChildren(tenantId, commentId);
}

fetchChildren();
[inline-code-end]

---