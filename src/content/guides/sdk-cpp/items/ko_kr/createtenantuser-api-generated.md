## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createTenantUserBody | CreateTenantUserBody | 예 |  |

## 응답

반환: [`CreateTenantUser_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantUser_200_response.h)

## 예제

[inline-code-attrs-start title = 'createTenantUser 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
CreateTenantUserBody createTenantUserBody;
createTenantUserBody.email = utility::conversions::to_string_t("new.user@company.com");
createTenantUserBody.displayName = boost::optional<utility::string_t>(utility::conversions::to_string_t("New User"));

api->createTenantUser(tenantId, createTenantUserBody)
    .then([](pplx::task<std::shared_ptr<CreateTenantUser_200_response>> task){
        try {
            auto resp = task.get();
            auto created = std::make_shared<CreateTenantUser_200_response>(*resp);
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---