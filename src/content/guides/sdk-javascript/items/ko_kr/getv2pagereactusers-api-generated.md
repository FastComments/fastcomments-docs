---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| id | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactUsersResponse.ts)

## 예시

[inline-code-attrs-start title = 'getV2PageReactUsers 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUsers() {
  const tenantId: string = "d4e5f6a7-89ab-4cde-f012-3456789abcde";
  const urlId: string = "blog/2024/06/fastcomments-typescript";
  const userId: string = "user_123456789";
  const ssoToken: string = "sso_token_abcdef123456";

  const responseWithSso: GetV2PageReactUsersResponse = await getV2PageReactUsers(tenantId, urlId, userId, ssoToken);
  const responseWithoutSso: GetV2PageReactUsersResponse = await getV2PageReactUsers(tenantId, urlId, userId);
}
[inline-code-end]

---