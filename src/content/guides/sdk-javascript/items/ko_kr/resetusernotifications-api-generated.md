## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| afterId | string | 아니오 |  |
| afterCreatedAt | number | 아니오 |  |
| unreadOnly | boolean | 아니오 |  |
| dmOnly | boolean | 아니오 |  |
| noDm | boolean | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## 예제

[inline-code-attrs-start title = 'resetUserNotifications 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_4a9f12";
const afterId: string = "notification_87213";
const afterCreatedAt: number = Math.floor(Date.now() / 1000) - 3600;
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.payload";
const result: ResetUserNotifications200Response = await resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, dmOnly, undefined, sso);
[inline-code-end]

---