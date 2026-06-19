## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| afterId | string | 否 |  |
| afterCreatedAt | number | 否 |  |
| unreadOnly | boolean | 否 |  |
| dmOnly | boolean | 否 |  |
| noDm | boolean | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## 範例

[inline-code-attrs-start title = 'resetUserNotifications 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-9f2b';
const afterId: string = 'notif_7c1a2b3';
const afterCreatedAt: number = Date.now() - 3 * 24 * 60 * 60 * 1000;
const unreadOnly: boolean = true;
const sso: string = 'sso:microsoft:84012';
const response: ResetUserNotificationsResponse = await resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, undefined, undefined, sso);
[inline-code-end]

---