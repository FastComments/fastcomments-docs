## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| skip | double | לא |  |

## תגובה

מחזיר: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateRenderErrorsResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getEmailTemplateRenderErrors'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("email-template-789");
boost::optional<double> skip = boost::optional<double>(10.0);
api->getEmailTemplateRenderErrors(tenantId, templateId, skip)
    .then([](pplx::task<std::shared_ptr<GetEmailTemplateRenderErrorsResponse>> t) {
        try {
            auto resp = t.get();
            auto safeResp = resp ? resp : std::make_shared<GetEmailTemplateRenderErrorsResponse>();
            (void)safeResp;
        } catch (const std::exception& e) {
            (void)e;
        }
    }).wait();
[inline-code-end]

---