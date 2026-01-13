## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantBody | UpdateTenantBody | 是 |  |

## 响应

返回: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 示例

[inline-code-attrs-start title = 'updateTenant 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto id = utility::string_t(U("tenant-0001"));
auto bodyPtr = std::make_shared<UpdateTenantBody>();
bodyPtr->setDisplayName(utility::string_t(U("Acme Corporation")));
bodyPtr->setAdminEmail(utility::string_t(U("admin@acme.com")));
bodyPtr->setIsActive(true);
bodyPtr->setMaxUsers(boost::optional<int>(250));
api->updateTenant(tenantId, id, *bodyPtr)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto msg = resp->getMessage();
            std::wcout << msg << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << e.what() << std::endl;
    }
});
[inline-code-end]

---