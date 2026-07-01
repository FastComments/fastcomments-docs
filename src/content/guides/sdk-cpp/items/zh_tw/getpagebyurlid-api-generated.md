## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## 回應

回傳：[`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPageByURLIdAPIResponse.h)

## 範例

[inline-code-attrs-start title = 'getPageByURLId 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto urlId = utility::conversions::to_string_t("page-456");
boost::optional<utility::string_t> correlationId = boost::make_optional(utility::conversions::to_string_t("corr-789"));

api->getPageByURLId(tenantId, urlId)
    .then([correlationId](pplx::task<std::shared_ptr<GetPageByURLIdAPIResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]