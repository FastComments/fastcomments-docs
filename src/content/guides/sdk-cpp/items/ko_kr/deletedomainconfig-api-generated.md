## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## 응답

반환: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteDomainConfigResponse.h)

## 예제

[inline-code-attrs-start title = 'deleteDomainConfig 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> optTenant = U("my-tenant-123");
boost::optional<utility::string_t> optDomain = U("example.com");

api->deleteDomainConfig(optTenant.value(), optDomain.value())
    .then([](pplx::task<std::shared_ptr<DeleteDomainConfigResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]