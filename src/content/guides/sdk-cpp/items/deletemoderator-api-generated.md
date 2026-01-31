## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| sendEmail | string | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteModerator Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t moderatorId = U("mod-456");
boost::optional<utility::string_t> sendEmail = boost::optional<utility::string_t>(U("notify@company.com"));
api->deleteModerator(tenantId, moderatorId, sendEmail)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task) {
    try {
        auto resp = task.get();
        if (resp) {
            auto acknowledged = std::make_shared<FlagCommentPublic_200_response>(*resp);
        }
    } catch (const std::exception& e) {
    }
}).wait();
[inline-code-end]
