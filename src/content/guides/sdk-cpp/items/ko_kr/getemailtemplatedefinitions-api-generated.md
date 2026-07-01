## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## 응답

반환: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateDefinitionsResponse.h)

## 예시

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
api->getEmailTemplateDefinitions(tenantId).then([](pplx::task<std::shared_ptr<GetEmailTemplateDefinitionsResponse>> t) {
    try {
        auto resp = t.get();
        boost::optional<utility::string_t> tmplName = resp ? resp->templateName : boost::none;
        if (tmplName) {
            std::cout << *tmplName << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << e.what() << std::endl;
    }
});
[inline-code-end]