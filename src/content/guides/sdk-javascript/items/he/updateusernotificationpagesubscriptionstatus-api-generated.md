Enable or disable notifications for a page. When users are subscribed to a page, notifications are created for new root comments, and also

## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| url | string | כן |  |
| pageTitle | string | כן |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | כן |  |
| sso | string | לא |  |

## Response

מחזיר: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת updateUserNotificationPageSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";
  const url: string = "https://example.com/articles/awesome-article";
  const pageTitle: string = "Awesome Article";
  const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum =
    UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
  const sso: string = "sso_token_abc";

  const response: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
    tenantId,
    urlId,
    url,
    pageTitle,
    subscribedOrUnsubscribed,
    sso
  );

  console.log(response);
}

runExample();
[inline-code-end]