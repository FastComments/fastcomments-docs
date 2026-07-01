## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentIds | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回：[`CheckedCommentsForBlockedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckedCommentsForBlockedResponse.ts)

## 示例

[inline-code-attrs-start title = 'checkedCommentsForBlocked 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const commentIds: string = "cmt_001,cmt_002";
  const ssoToken: string = "ssoTokenXYZ";

  const blockedCheck: CheckedCommentsForBlockedResponse = await checkedCommentsForBlocked(tenantId, commentIds);
  const blockedCheckWithSso: CheckedCommentsForBlockedResponse = await checkedCommentsForBlocked(tenantId, commentIds, ssoToken);
})();
[inline-code-end]