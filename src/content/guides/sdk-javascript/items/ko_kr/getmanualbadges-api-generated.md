## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantManualBadgesResponse.ts)

## 예시

[inline-code-attrs-start title = 'getManualBadges 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadges() {
  const tenantId: string = "tenant-987654321";
  const ssoToken: string = "sso-token-abc123";

  const badgesWithSso: GetTenantManualBadgesResponse = await getManualBadges(tenantId, ssoToken);
  const badgesWithoutSso: GetTenantManualBadgesResponse = await getManualBadges(tenantId);
}
[inline-code-end]