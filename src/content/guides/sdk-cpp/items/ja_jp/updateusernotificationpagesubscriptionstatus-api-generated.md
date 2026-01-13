ページの通知を有効化または無効化します。ユーザーがページを購読している場合、通知は作成され
新しいルートコメントに対して、また

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| url | string | はい |  |
| pageTitle | string | はい |  |
| subscribedOrUnsubscribed | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`UpdateUserNotificationStatus_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatus_200_response.h)

## 例

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
utility::string_t url = U("https://www.example.com/articles/2026/new-feature");
utility::string_t pageTitle = U("New Feature Announcement");
utility::string_t subscribedOrUnsubscribed = U("subscribed");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto placeholder = std::make_shared<UpdateUserNotificationStatus_200_response>();
api->updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso)
.then([](pplx::task<std::shared_ptr<UpdateUserNotificationStatus_200_response>> task){
    try {
        auto resp = task.get();
        (void)resp;
    } catch (const std::exception &e) {
        (void)e;
    }
});
[inline-code-end]