## 매개변수

| Name | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantPackageBody | UpdateTenantPackageBody | 예 |  |

## 응답

반환: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 예제

[inline-code-attrs-start title = 'updateTenantPackage 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t packageId = U("pkg-456");
auto bodyPtr = std::make_shared<UpdateTenantPackageBody>();
bodyPtr->name = utility::string_t(U("Pro Monthly"));
bodyPtr->description = boost::optional<utility::string_t>(U("Monthly subscription for pro users"));
bodyPtr->enabled = boost::optional<bool>(true);
api->updateTenantPackage(tenantId, packageId, *bodyPtr)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    std::cout << (resp ? "updateTenantPackage succeeded\n" : "updateTenantPackage returned null\n");
});
[inline-code-end]

---