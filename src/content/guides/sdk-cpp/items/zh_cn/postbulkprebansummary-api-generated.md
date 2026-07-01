## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| bulkPreBanParams | BulkPreBanParams | 是 |  |
| options | const PostBulkPreBanSummaryOptions& | 是 |  |

## 响应

返回: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkPreBanSummary.h)

## 示例

[inline-code-attrs-start title = 'postBulkPreBanSummary 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
BulkPreBanParams bulkPreBanParams;
bulkPreBanParams.emails = {
    utility::conversions::to_string_t("spam1@example.com"),
    utility::conversions::to_string_t("spam2@example.com")
};
bulkPreBanParams.reason = utility::conversions::to_string_t("spam");
PostBulkPreBanSummaryOptions options;
options.requestId = boost::optional<utility::string_t>(utility::conversions::to_string_t("req-456"));
api->postBulkPreBanSummary(tenantId, bulkPreBanParams, options)
    .then([](std::shared_ptr<BulkPreBanSummary> result) {
        (void)result;
    });
[inline-code-end]