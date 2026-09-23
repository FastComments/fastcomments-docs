---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse.ts)

## 예시

[inline-code-attrs-start title = 'getUserNotificationCount 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo(): Promise<void> {
  const tenantId: string = "acme-corp-001";
  const ssoToken: string = "sso-token-xyz789";

  const countWithoutSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId);
  const countWithSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId, ssoToken);
}

demo();
[inline-code-end]

---