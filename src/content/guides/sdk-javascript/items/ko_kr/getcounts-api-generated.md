## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## 응답

반환: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersCountResponse.ts)

## 예시

[inline-code-attrs-start title = 'getCounts 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "tenant_9876";
  const ssoToken: string = "sso_user_42";

  const resultWithOnlyTenant: GetBannedUsersCountResponse = await getCounts(tenantId);
  const resultWithBoth: GetBannedUsersCountResponse = await getCounts(tenantId, ssoToken);
}

run();
[inline-code-end]

---