啟用或停用頁面的通知。當使用者訂閱頁面時，會為新的根留言建立通知，並且也

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| url | string | 是 |  |
| pageTitle | string | 是 |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | 是 |  |
| sso | string | 否 |  |

## 回應

回傳：[`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "8a3f2b1c-4d6e-4f9b-9c2d-0a1b2c3d4e5f";
const urlId: string = "article-2026-reliable-api";
const url: string = "https://blog.companyexample.com/articles/reliable-api-patterns";
const pageTitle: string = "Reliable API Patterns for Integrations";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload";
const result: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
  tenantId,
  urlId,
  url,
  pageTitle,
  UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed,
  sso
);
[inline-code-end]

---