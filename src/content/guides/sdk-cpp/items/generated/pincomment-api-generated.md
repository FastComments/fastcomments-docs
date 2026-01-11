## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`PinComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PinComment_200_response.h)

## Example

[inline-code-attrs-start title = 'pinComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso(U("user@example.com"));
api->pinComment(U("my-tenant-123"), U("cmt-456"), U("broadcast-789"), sso)
.then([](pplx::task<std::shared_ptr<PinComment_200_response>> task){
    try {
        auto resp = task.get();
        auto result = resp ? resp : std::make_shared<PinComment_200_response>();
        std::cout << "pinComment completed successfully\n";
    } catch (const std::exception& e) {
        std::cerr << "pinComment failed: " << e.what() << '\n';
    }
});
[inline-code-end]
