## Parameters

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| domainToUpdate | string | 예 |  |
| updateDomainConfigParams | UpdateDomainConfigParams | 예 |  |

## 응답

반환: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## 예제

[inline-code-attrs-start title = 'putDomainConfig 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("comments.myapp.com");
auto params = std::make_shared<UpdateDomainConfigParams>();
params->displayName = utility::string_t(U("My App Comments"));
params->enabled = boost::optional<bool>(true);
params->contactEmail = boost::optional<utility::string_t>(U("admin@myapp.com"));
api->putDomainConfig(tenantId, domainToUpdate, *params)
.then([](pplx::task<std::shared_ptr<GetDomainConfig_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto updated = resp;
        }
    } catch (const std::exception& e) {
    }
});
[inline-code-end]

---