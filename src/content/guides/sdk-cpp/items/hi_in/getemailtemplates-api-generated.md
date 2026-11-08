## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | double | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetEmailTemplatesResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getEmailTemplates उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> skip = 10.0;
api->getEmailTemplates(tenantId, skip)
    .then([](std::shared_ptr<GetEmailTemplatesResponse> resp) {
        (void)resp;
    });
[inline-code-end]