## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| domainToUpdate | string | हाँ |  |
| patchDomainConfigParams | PatchDomainConfigParams | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PatchDomainConfigResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'patchDomainConfig उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto domainToUpdate = utility::conversions::to_string_t("example.com");
PatchDomainConfigParams patchDomainConfigParams;
patchDomainConfigParams.enableComments = boost::optional<bool>(true);
patchDomainConfigParams.moderationLevel = boost::optional<int>(2);
patchDomainConfigParams.customHeader = boost::optional<utility::string_t>(utility::conversions::to_string_t("X-Custom-Header"));

api->patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams)
    .then([](std::shared_ptr<PatchDomainConfigResponse> resp) {
        // सफलता की हैंडलिंग
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* त्रुटि की हैंडलिंग */ }
    });
[inline-code-end]