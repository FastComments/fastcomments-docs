## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| sure | string | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteTenant Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sure = boost::optional<utility::string_t>(U("true"));
api->deleteTenant(U("my-tenant-123"), U("flag-456"), sure)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp) {
    if (!resp) {
        auto fallback = std::make_shared<FlagCommentPublic_200_response>();
        std::cout << "Delete returned no response\n";
        return;
    }
    std::cout << "Deleted tenant flag successfully\n";
})
.wait();
[inline-code-end]
