## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| domain | string | 예 |  |

## 응답

반환: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteDomainConfigResponse.h)

## 예제

[inline-code-attrs-start title = 'deleteDomainConfig 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("example.com");
boost::optional<utility::string_t> correlationId = boost::optional<utility::string_t>(U("corr-789"));
api->deleteDomainConfig(tenantId, domain).then([correlationId](pplx::task<std::shared_ptr<DeleteDomainConfigResponse>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<DeleteDomainConfigResponse>();
        utility::string_t cid = correlationId ? *correlationId : U("");
        (void)cid;
    } catch(const std::exception &){
        auto err = std::make_shared<DeleteDomainConfigResponse>();
        (void)err;
    }
});
[inline-code-end]

---