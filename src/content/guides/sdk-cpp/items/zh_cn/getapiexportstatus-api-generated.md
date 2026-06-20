## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| batchJobId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationExportStatusResponse.h)

## 示例

[inline-code-attrs-start title = 'getApiExportStatus 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> batchId = utility::string_t(U("export-batch-2026-06-19"));
boost::optional<utility::string_t> sso = utility::string_t(U("audit@my-tenant-123.com"));
api->getApiExportStatus(batchId, sso)
    .then([](std::shared_ptr<ModerationExportStatusResponse> resp) {
        if (!resp) return;
        auto statusCopy = std::make_shared<ModerationExportStatusResponse>(*resp);
    });
[inline-code-end]

---