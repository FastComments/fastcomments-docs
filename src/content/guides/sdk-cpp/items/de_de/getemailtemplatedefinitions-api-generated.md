## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |

## Antwort

Rückgabe: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateDefinitionsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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