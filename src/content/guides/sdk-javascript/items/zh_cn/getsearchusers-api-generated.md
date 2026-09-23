## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| value | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationUserSearchResponse.ts)

## 示例

[inline-code-attrs-start title = 'getSearchUsers 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runSearch() {
  const tenantId: string = "tenant-9876";
  const emailFragment: string = "jane";
  const ssoToken: string = "sso-token-456";

  const resultWithAll: ModerationUserSearchResponse = await getSearchUsers(tenantId, emailFragment, ssoToken);
  const resultWithTenantOnly: ModerationUserSearchResponse = await getSearchUsers(tenantId);
}
[inline-code-end]