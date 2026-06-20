## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| replaceTenantUserBody | ReplaceTenantUserBody | 예 |  |
| updateComments | string | 아니요 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'replaceTenantUser 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t(U("my-tenant-123"));
utility::string_t userId = utility::string_t(U("user@example.com"));
auto body = std::make_shared<ReplaceTenantUserBody>();
body->email = utility::string_t(U("user@example.com"));
body->name = utility::string_t(U("Jane Doe"));
body->role = utility::string_t(U("member"));
boost::optional<utility::string_t> updateComments = boost::optional<utility::string_t>(utility::string_t(U("true")));
api->replaceTenantUser(tenantId, userId, *body, updateComments)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try {
        auto resp = task.get();
        (void)resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---