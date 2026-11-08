## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetAuditLogsOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetAuditLogsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getAuditLogs उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");

GetAuditLogsOptions options;
options.startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-01T00:00:00Z"), utility::datetime::RFC_1123));
options.endDate   = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-31T23:59:59Z"), utility::datetime::RFC_1123));
options.limit     = boost::optional<int>(100);

api->getAuditLogs(tenantId, options).then([](pplx::task<std::shared_ptr<GetAuditLogsResponse>> t) {
    try {
        auto response = t.get();
        // response का उपयोग करें
    } catch (const std::exception& e) {
        // त्रुटि को संभालें
    }
});
[inline-code-end]