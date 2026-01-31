## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | Yes |  |

## Response

Returns: [`CreateUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateUserBadge_200_response.h)

## Example

[inline-code-attrs-start title = 'createUserBadge Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto paramsPtr = std::make_shared<CreateUserBadgeParams>();
paramsPtr->setUserId(U("user@example.com"));
paramsPtr->setBadgeId(U("top-contributor"));
paramsPtr->setReason(boost::optional<utility::string_t>(U("Recognized for insightful comments")));
api->createUserBadge(tenantId, *paramsPtr).then([](pplx::task<std::shared_ptr<CreateUserBadge_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) std::cout << "Created badge id: " << resp->getId() << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Error creating badge: " << e.what() << std::endl;
    }
});
[inline-code-end]
