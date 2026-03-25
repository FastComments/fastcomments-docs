## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | 是 |  |
| userId | string | 否 |  |

## 回應

回傳: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateSubscriptionAPIResponse.h)

## 範例

[inline-code-attrs-start title = 'updateSubscription 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t subscriptionId = U("sub-456");
UpdateAPIUserSubscriptionData updateData;
auto fallbackResp = std::make_shared<UpdateSubscriptionAPIResponse>();
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
api->updateSubscription(tenantId, subscriptionId, updateData, userId)
    .then([](pplx::task<std::shared_ptr<UpdateSubscriptionAPIResponse>> task) {
        try {
            auto resp = task.get();
            if (resp) {
                // 處理 resp
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---