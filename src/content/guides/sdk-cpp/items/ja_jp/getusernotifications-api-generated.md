---
## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | いいえ |  |
| pageSize | int32_t | いいえ |  |
| afterId | string | いいえ |  |
| includeContext | bool | いいえ |  |
| afterCreatedAt | int64_t | いいえ |  |
| unreadOnly | bool | いいえ |  |
| dmOnly | bool | いいえ |  |
| noDm | bool | いいえ |  |
| includeTranslations | bool | いいえ |  |
| includeTenantNotifications | bool | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## 例

[inline-code-attrs-start title = 'getUserNotifications の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
api->getUserNotifications(
    tenantId,
    boost::optional<utility::string_t>(U("post-456")),
    boost::optional<int32_t>(50),
    boost::optional<utility::string_t>(U("notif-789")),
    boost::optional<bool>(true),
    boost::optional<int64_t>(1625097600000LL),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<bool>(false),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<utility::string_t>(U("user@example.com"))
).then([](pplx::task<std::shared_ptr<GetMyNotificationsResponse>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<GetMyNotificationsResponse>();
        // resp を使用、例：フィールドを確認
    } catch(const std::exception &e) {
    }
});
[inline-code-end]

---