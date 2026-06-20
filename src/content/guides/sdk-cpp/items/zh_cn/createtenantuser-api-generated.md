---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantUserBody | CreateTenantUserBody | 是 |  |

## 响应

返回：[`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantUserResponse.h)

## 示例

[inline-code-attrs-start title = 'createTenantUser 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateTenantUserBody body;
body.email = utility::string_t(U("jane.doe@example.com"));
body.displayName = utility::string_t(U("Jane Doe"));
body.role = utility::string_t(U("moderator"));
body.sendInvite = boost::optional<bool>(true);
api->createTenantUser(tenantId, body).then([](std::shared_ptr<CreateTenantUserResponse> resp) {
    if (resp) {
        auto created = std::make_shared<CreateTenantUserResponse>(*resp);
        std::cout << "Created tenant user successfully" << std::endl;
    } else {
        std::cout << "Create tenant user returned null" << std::endl;
    }
});
[inline-code-end]

---