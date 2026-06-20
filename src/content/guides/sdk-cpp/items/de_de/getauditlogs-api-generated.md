---
## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| limit | double | Nein |  |
| skip | double | Nein |  |
| order | SORT_DIR | Nein |  |
| after | double | Nein |  |
| before | double | Nein |  |

## Antwort

Gibt zurück: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetAuditLogsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getAuditLogs Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> limit = 100.0;
boost::optional<double> skip = 0.0;
boost::optional<SORT_DIR> order = SORT_DIR::DESC;
boost::optional<double> after;
boost::optional<double> before;
api->getAuditLogs(tenantId, limit, skip, order, after, before)
.then([](pplx::task<std::shared_ptr<GetAuditLogsResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetAuditLogsResponse>();
        std::cout << "Fetched audit logs for tenant\n";
    } catch (const std::exception &e) {
        std::cerr << "getAuditLogs failed: " << e.what() << '\n';
    }
});
[inline-code-end]

---