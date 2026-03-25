## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Yes |  |
| userId | string | No |  |

## Response

Returns: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateSubscriptionAPIResponse.h)

## Example

[inline-code-attrs-start title = 'updateSubscription Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
                // process resp
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
