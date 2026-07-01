## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## 回應

回傳: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSubscriptionsAPIResponse.h)

## 範例

[inline-code-attrs-start title = 'getSubscriptions 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenant = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> user = utility::conversions::to_string_t("user@example.com");

api->getSubscriptions(tenant, user).then(
    [](pplx::task<std::shared_ptr<GetSubscriptionsAPIResponse>> t) {
        try {
            auto response = t.get();
            // 處理回應
        } catch (const std::exception& e) {
            // 處理錯誤
        }
    }
);
[inline-code-end]