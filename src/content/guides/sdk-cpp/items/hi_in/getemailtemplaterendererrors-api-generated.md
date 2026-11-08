## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| skip | double | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetEmailTemplateRenderErrorsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("email-template-789");
boost::optional<double> skip = 20.0;

api->getEmailTemplateRenderErrors(tenantId, id, skip)
    .then([](pplx::task<std::shared_ptr<GetEmailTemplateRenderErrorsResponse>> task) {
        try {
            auto response = task.get();
            // प्रतिक्रिया का आवश्यकतानुसार उपयोग करें
        } catch (const std::exception& ex) {
            // त्रुटि को संभालें
        }
    });
[inline-code-end]