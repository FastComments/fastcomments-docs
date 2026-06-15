ページの通知を有効または無効にします。ユーザーがページを購読している場合、新しいルートコメントに対して通知が作成され、また

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| url | string | はい |  |
| pageTitle | string | はい |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

## 例

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b2';
const urlId: string = 'article_987';
const url: string = 'https://www.news-site.com/articles/2026/pasta-guide';
const pageTitle: string = 'The Definitive Guide to Cooking Pasta';
const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum = UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
const sso: string = 'sso-token-62b9f1';
const result: UpdateUserNotificationStatus200Response = await updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso);
[inline-code-end]

---