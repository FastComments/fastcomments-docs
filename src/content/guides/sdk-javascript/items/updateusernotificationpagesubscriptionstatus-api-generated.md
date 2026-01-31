Enable or disable notifications for a page. When users are subscribed to a page, notifications are created
for new root comments, and also

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| url | string | Yes |  |
| pageTitle | string | Yes |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Yes |  |
| sso | string | No |  |

## Response

Returns: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## Example

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f2b";
const urlId: string = "page_93a1";
const url: string = "https://www.news-site.com/opinion/ai-moderation-2026";
const pageTitle: string = "AI Moderation: What to Expect in 2026";
const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum =
  UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
const sso: string = "sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

const result: UpdateUserNotificationStatus200Response = await updateUserNotificationPageSubscriptionStatus(
  tenantId,
  urlId,
  url,
  pageTitle,
  subscribedOrUnsubscribed,
  sso
);
[inline-code-end]
