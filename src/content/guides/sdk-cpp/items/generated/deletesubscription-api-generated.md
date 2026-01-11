## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |

## Response

Returns: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSubscriptionAPIResponse.h)

## Example

[inline-code-attrs-start title = 'deleteSubscription Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t subscriptionId = U("sub-987654");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));

api->deleteSubscription(tenantId, subscriptionId, userId)
.then([](pplx::task<std::shared_ptr<DeleteSubscriptionAPIResponse>> task) {
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<DeleteSubscriptionAPIResponse>();
        std::cout << "Subscription deleted for tenant\n";
    } catch (const std::exception& e) {
        std::cerr << "Error deleting subscription: " << e.what() << '\n';
    }
});
[inline-code-end]
