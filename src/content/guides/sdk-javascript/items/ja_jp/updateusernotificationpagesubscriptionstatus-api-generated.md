---
ページの通知を有効または無効にします。ユーザーがページを購読している場合、通知が作成されます
新しいルートコメントに対して、そしてまた

## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| url | string | はい |  |
| pageTitle | string | はい |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## 例

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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