## 參數

| 名稱 | Type | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| afterId | string | 否 |  |
| afterCreatedAt | number | 否 |  |
| unreadOnly | boolean | 否 |  |
| dmOnly | boolean | 否 |  |
| noDm | boolean | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## 範例

[inline-code-attrs-start title = 'resetUserNotifications 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_4a9f12";
const afterId: string = "notification_87213";
const afterCreatedAt: number = Math.floor(Date.now() / 1000) - 3600;
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.payload";
const result: ResetUserNotifications200Response = await resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, dmOnly, undefined, sso);
[inline-code-end]