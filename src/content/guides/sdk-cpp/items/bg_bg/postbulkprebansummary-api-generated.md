## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| bulkPreBanParams | BulkPreBanParams | Да |  |
| options | const PostBulkPreBanSummaryOptions& | Да |  |

## Отговор

Връща: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkPreBanSummary.h)

## Пример

[inline-code-attrs-start title = 'postBulkPreBanSummary пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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