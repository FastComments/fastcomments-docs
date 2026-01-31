## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | Yes |  |

## Response

Returns: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateSubscriptionAPIResponse.h)

## Example

[inline-code-attrs-start title = 'createSubscription Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto req = std::make_shared<CreateAPIUserSubscriptionData>();
req->userEmail = utility::string_t(U("alice@example.com"));
req->planId = utility::string_t(U("pro-monthly"));
req->autoRenew = boost::optional<bool>(true);
req->metadata = boost::optional<utility::string_t>(U("campaign=summer2025"));

api->createSubscription(tenantId, *req)
    .then([](std::shared_ptr<CreateSubscriptionAPIResponse> resp){
        if (resp) std::cout << "Subscription created" << std::endl;
    });
[inline-code-end]
