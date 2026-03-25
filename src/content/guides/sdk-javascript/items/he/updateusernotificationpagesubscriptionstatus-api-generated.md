הפעל או השבת התראות עבור עמוד. כאשר משתמשים מנויים לעמוד, ייווצרו התראות עבור תגובות שורש חדשות, וגם

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| url | string | כן |  |
| pageTitle | string | כן |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של updateUserNotificationPageSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const urlId: string = 'blog-launch-2026';
const url: string = 'https://acme.example.com/blog/launch-march-2026';
const pageTitle: string = 'Acme Product Launch — March 2026';
const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum = UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
const sso: string = 'sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const response: UpdateUserNotificationStatus200Response = await updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso);
[inline-code-end]

---