---
啟用或停用頁面的通知。當使用者訂閱頁面時，會為新的頂層評論建立通知，並且也

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| url | string | 是 |  |
| pageTitle | string | 是 |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

---