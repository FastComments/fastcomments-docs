## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantUserBody | UpdateTenantUserBody | 예 |  |
| updateComments | string | 아니요 |  |

## 응답

반환: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 예제

[inline-code-attrs-start title = 'updateTenantUser 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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