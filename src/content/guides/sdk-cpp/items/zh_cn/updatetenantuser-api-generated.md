## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantUserBody | UpdateTenantUserBody | 是 |  |
| updateComments | string | 否 |  |

## 响应

返回： [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 示例

[inline-code-attrs-start title = 'updateTenantUser 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("jane.doe@example.com");
auto bodyPtr = std::make_shared<UpdateTenantUserBody>();
bodyPtr->setEmail(U("jane.doe@example.com"));
bodyPtr->setDisplayName(U("Jane Doe"));
boost::optional<utility::string_t> updateComments = U("Normalized display name");

api->updateTenantUser(tenantId, userId, *bodyPtr, updateComments)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Tenant user updated successfully\n";
        else std::cout << "Update returned no data\n";
    } catch (const std::exception& ex) {
        std::cerr << "Update failed: " << ex.what() << '\n';
    }
});
[inline-code-end]

---