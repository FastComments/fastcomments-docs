## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回：[`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## 示例

[inline-code-attrs-start title = 'pinComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoPinComment() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_54321";

  const resultWithoutSso: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId);
  const ssoToken: string = "sso_abcde12345";
  const resultWithSso: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId, ssoToken);
}

demoPinComment();
[inline-code-end]

---