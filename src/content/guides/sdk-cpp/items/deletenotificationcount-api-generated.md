## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteNotificationCount Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t notificationId = U("notify-42a7c");
boost::optional<int> timeoutMs = 5000;
api->deleteNotificationCount(tenantId, notificationId)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copy = std::make_shared<FlagCommentPublic_200_response>(*resp);
            std::cout << "Delete succeeded\n";
        } else {
            std::cout << "Empty response\n";
        }
    } catch (const std::exception &e) {
        std::cerr << "API error: " << e.what() << '\n';
    }
});
[inline-code-end]
