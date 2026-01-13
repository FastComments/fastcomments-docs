## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| domainToUpdate | string | 예 |  |
| patchDomainConfigParams | PatchDomainConfigParams | 예 |  |

## 응답

반환: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## 예제

[inline-code-attrs-start title = 'patchDomainConfig 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("example.com");
PatchDomainConfigParams params;
params.adminEmail = boost::optional<utility::string_t>(U("admin@example.com"));
params.enableSso = boost::optional<bool>(true);
params.ssoRedirectUrl = boost::optional<utility::string_t>(U("https://auth.example.com/callback"));
api->patchDomainConfig(tenantId, domainToUpdate, params)
    .then([](pplx::task<std::shared_ptr<GetDomainConfig_200_response>> task){
        try {
            auto resp = task.get();
            if (resp) {
                auto updated = std::make_shared<GetDomainConfig_200_response>(*resp);
            }
        } catch (const std::exception& e) {
            auto err = std::string(e.what());
        }
    });
[inline-code-end]

---