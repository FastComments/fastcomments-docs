## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |

## 응답

반환값: [`GetEmailTemplateDefinitions_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateDefinitions_200_response.h)

## 예제

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantIdOpt = utility::string_t(U("my-tenant-123"));
api->getEmailTemplateDefinitions(tenantIdOpt.value())
.then([](pplx::task<std::shared_ptr<GetEmailTemplateDefinitions_200_response>> task) {
    try {
        auto resp = task.get();
        auto localCopy = std::make_shared<GetEmailTemplateDefinitions_200_response>(*resp);
        (void)localCopy;
    } catch (const std::exception& e) {
        auto fallback = std::make_shared<GetEmailTemplateDefinitions_200_response>();
        (void)fallback;
    }
});
[inline-code-end]