## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 응답

반환: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetCachedNotificationCountResponse.h)

## 예제

[inline-code-attrs-start title = 'getCachedNotificationCount 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = boost::optional<utility::string_t>(utility::conversions::to_string_t("my-tenant-123"));
auto userId = boost::optional<utility::string_t>(utility::conversions::to_string_t("user-456"));

api->getCachedNotificationCount(tenantId.value(), userId.value())
    .then([](pplx::task<std::shared_ptr<GetCachedNotificationCountResponse>> task) {
        try {
            auto response = task.get();
            // 응답 처리
        } catch (const std::exception&) {
            // 오류 처리
        }
    });
[inline-code-end]

---