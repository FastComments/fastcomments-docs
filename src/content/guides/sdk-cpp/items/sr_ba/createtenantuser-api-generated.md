## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createTenantUserBody | CreateTenantUserBody | Да |  |

## Одговор

Враћа: [`CreateTenantUser_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantUser_200_response.h)

## Пример

[inline-code-attrs-start title = 'createTenantUser Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
CreateTenantUserBody createTenantUserBody;
createTenantUserBody.email = utility::conversions::to_string_t("new.user@company.com");
createTenantUserBody.password = utility::conversions::to_string_t("Secur3P@ssw0rd!");
createTenantUserBody.displayName = boost::optional<utility::string_t>(utility::conversions::to_string_t("New User"));
createTenantUserBody.roles = std::vector<utility::string_t>{utility::conversions::to_string_t("member")};

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