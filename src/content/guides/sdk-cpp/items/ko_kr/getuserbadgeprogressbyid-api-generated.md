---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeProgressResponse.h)

## 예제

[inline-code-attrs-start title = 'getUserBadgeProgressById 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t userId = utility::conversions::to_string_t("alice@acme.com");
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(utility::conversions::to_string_t("en-US"));
auto task = api->getUserBadgeProgressById(tenantId, userId)
    .then([locale](pplx::task<std::shared_ptr<APIGetUserBadgeProgressResponse>> t) -> std::shared_ptr<APIGetUserBadgeProgressResponse> {
        try {
            std::shared_ptr<APIGetUserBadgeProgressResponse> resp = t.get();
            if (!resp) return std::shared_ptr<APIGetUserBadgeProgressResponse>();
            auto result = std::make_shared<APIGetUserBadgeProgressResponse>(*resp);
            if (locale) { auto lang = *locale; (void)lang; }
            return result;
        } catch (...) {
            return std::shared_ptr<APIGetUserBadgeProgressResponse>();
        }
    });
[inline-code-end]

---