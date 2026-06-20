## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |

## תגובה

מחזיר: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateDefinitionsResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getEmailTemplateDefinitions'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> locale = boost::none;
api->getEmailTemplateDefinitions(tenantId)
.then([=](pplx::task<std::shared_ptr<GetEmailTemplateDefinitionsResponse>> task) {
    try {
        auto resp = task.get();
        auto safeResp = resp ? resp : std::make_shared<GetEmailTemplateDefinitionsResponse>();
        return safeResp;
    } catch (const std::exception&) {
        return std::make_shared<GetEmailTemplateDefinitionsResponse>();
    }
});
[inline-code-end]

---