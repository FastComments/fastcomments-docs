## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const PostApiExportOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ModerationExportResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'postApiExport उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
PostApiExportOptions options;
options.format = utility::string_t(U("json"));
options.email = utility::string_t(U("reports@example.com"));
options.startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-01T00:00:00Z")));
options.endDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-31T23:59:59Z")));

api->postApiExport(tenantId, options)
    .then([](std::shared_ptr<ModerationExportResponse> response) {
        if (response) {
            // सफल एक्सपोर्ट प्रतिक्रिया को संभालें
        }
    })
    .wait();
[inline-code-end]