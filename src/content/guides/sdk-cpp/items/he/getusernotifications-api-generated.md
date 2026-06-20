## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | לא |  |
| pageSize | int32_t | לא |  |
| afterId | string | לא |  |
| includeContext | bool | לא |  |
| afterCreatedAt | int64_t | לא |  |
| unreadOnly | bool | לא |  |
| dmOnly | bool | לא |  |
| noDm | bool | לא |  |
| includeTranslations | bool | לא |  |
| includeTenantNotifications | bool | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
        // השתמש ב-resp, לדוגמה בדוק שדות
    } catch(const std::exception &e) {
    }
});
[inline-code-end]

---