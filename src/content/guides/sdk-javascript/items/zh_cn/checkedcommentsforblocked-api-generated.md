## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentIds | string | Yes |  |
| sso | string | No |  |

## 响应

返回：[`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckBlockedCommentsResponse.ts)

## 示例

[inline-code-attrs-start title = 'checkedCommentsForBlocked 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "tenant_12345";
    const commentIds: string = "cmt_9876,cmt_5432";
    const ssoToken: string = "sso_user_abc123";

    const responseWithSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds, ssoToken);
    const responseWithoutSso: CheckBlockedCommentsResponse = await checkedCommentsForBlocked(tenantId, commentIds);
})();
[inline-code-end]