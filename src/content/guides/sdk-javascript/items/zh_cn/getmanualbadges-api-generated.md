## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantManualBadgesResponse.ts)

## 示例

[inline-code-attrs-start title = 'getManualBadges 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadges() {
  const tenantId: string = "tenant-987654321";
  const ssoToken: string = "sso-token-abc123";

  const badgesWithSso: GetTenantManualBadgesResponse = await getManualBadges(tenantId, ssoToken);
  const badgesWithoutSso: GetTenantManualBadgesResponse = await getManualBadges(tenantId);
}
[inline-code-end]

---