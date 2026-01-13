## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| limit | double | 否 |  |
| skip | double | 否 |  |
| order | SORT_DIR | 否 |  |
| after | double | 否 |  |
| before | double | 否 |  |

## 回應

回傳: [`GetAuditLogs_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetAuditLogs_200_response.h)

## 範例

[inline-code-attrs-start title = 'getAuditLogs 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> limit = 100;
boost::optional<double> skip = 0;
boost::optional<SORT_DIR> order = SORT_DIR::DESC;
boost::optional<double> after = 1633046400;
boost::optional<double> before = 1633132800;
api->getAuditLogs(tenantId, limit, skip, order, after, before)
.then([](pplx::task<std::shared_ptr<GetAuditLogs_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<GetAuditLogs_200_response>(*resp);
            (void)copy;
        }
    } catch (...) {
    }
});
[inline-code-end]

---