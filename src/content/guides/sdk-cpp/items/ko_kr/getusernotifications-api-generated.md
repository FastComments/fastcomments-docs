---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 아니오 |  |
| pageSize | int32_t | 아니오 |  |
| afterId | string | 아니오 |  |
| includeContext | bool | 아니오 |  |
| afterCreatedAt | int64_t | 아니오 |  |
| unreadOnly | bool | 아니오 |  |
| dmOnly | bool | 아니오 |  |
| noDm | bool | 아니오 |  |
| includeTranslations | bool | 아니오 |  |
| includeTenantNotifications | bool | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## 예제

[inline-code-attrs-start title = 'getUserNotifications 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
        // resp 사용, 예: 필드 검사
    } catch(const std::exception &e) {
    }
});
[inline-code-end]

---