## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createTenantUserBody | CreateTenantUserBody | 예 |  |

## 응답

반환: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantUserResponse.h)

## 예시

[inline-code-attrs-start title = 'createTenantUser 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto body = CreateTenantUserBody{};
body.email = utility::conversions::to_string_t("newuser@example.com");
body.firstName = utility::conversions::to_string_t("Alice");
body.lastName = utility::conversions::to_string_t("Smith");
body.role = boost::optional<utility::string_t>(utility::conversions::to_string_t("moderator"));

api->createTenantUser(utility::conversions::to_string_t("my-tenant-123"), body)
    .then([](pplx::task<std::shared_ptr<CreateTenantUserResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---