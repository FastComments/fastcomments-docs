## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## תשובה

מחזירה: [`ResetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationCountResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת resetUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoResetCount() {
  const tenantId: string = "acme-corp-tenant";
  const sso: string = "sso-user-9876";

  // קריאה עם פרמטר sso אופציונלי
  const resultWithSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId, sso);

  // קריאה ללא פרמטר sso אופציונלי
  const resultWithoutSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId);

  console.log(resultWithSso, resultWithoutSso);
}
[inline-code-end]

---