## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| limit | double | いいえ |  |
| skip | double | いいえ |  |
| order | SORT_DIR | いいえ |  |
| after | double | いいえ |  |
| before | double | いいえ |  |

## レスポンス

戻り値: [`GetAuditLogs_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetAuditLogs_200_response.h)

## 例

[inline-code-attrs-start title = 'getAuditLogs の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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