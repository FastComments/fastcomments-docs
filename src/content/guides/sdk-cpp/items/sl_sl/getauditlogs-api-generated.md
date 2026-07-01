## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetAuditLogsOptions& | Yes |  |

## Odgovor

Vrne: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetAuditLogsResponse.h)

## Primer

[inline-code-attrs-start title = 'getAuditLogs Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");

GetAuditLogsOptions options;
options.startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-01T00:00:00Z"), utility::datetime::RFC_1123));
options.endDate   = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-31T23:59:59Z"), utility::datetime::RFC_1123));
options.limit     = boost::optional<int>(100);

api->getAuditLogs(tenantId, options).then([](pplx::task<std::shared_ptr<GetAuditLogsResponse>> t) {
    try {
        auto response = t.get();
        // uporabi odgovor
    } catch (const std::exception& e) {
        // obravnavaj napako
    }
});
[inline-code-end]