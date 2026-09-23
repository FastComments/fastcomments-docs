## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantManualBadgesResponse.ts)

## 例

[inline-code-attrs-start title = 'getManualBadges の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadges() {
  const tenantId: string = "tenant-987654321";
  const ssoToken: string = "sso-token-abc123";

  const badgesWithSso: GetTenantManualBadgesResponse = await getManualBadges(tenantId, ssoToken);
  const badgesWithoutSso: GetTenantManualBadgesResponse = await getManualBadges(tenantId);
}
[inline-code-end]

---