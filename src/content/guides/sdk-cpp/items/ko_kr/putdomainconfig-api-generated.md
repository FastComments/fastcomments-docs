## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| domainToUpdate | string | 예 |  |
| updateDomainConfigParams | UpdateDomainConfigParams | 예 |  |

## 응답

반환: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutDomainConfigResponse.h)

## 예제

[inline-code-attrs-start title = 'putDomainConfig 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("example.com");
boost::optional<utility::string_t> contactEmail = U("admin@example.com");
boost::optional<bool> enforceHttps = true;
UpdateDomainConfigParams updateParams;
updateParams.contactEmail = contactEmail;
updateParams.enforceHttps = enforceHttps;
api->putDomainConfig(tenantId, domainToUpdate, updateParams)
.then([](pplx::task<std::shared_ptr<PutDomainConfigResponse>> t){
    try {
        auto resp = t.get();
        return resp ? resp : std::make_shared<PutDomainConfigResponse>();
    } catch(...) {
        return std::make_shared<PutDomainConfigResponse>();
    }
});
[inline-code-end]

---