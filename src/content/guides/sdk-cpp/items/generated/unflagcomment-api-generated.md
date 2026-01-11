## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`FlagComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagComment_200_response.h)

## Example

[inline-code-attrs-start title = 'unFlagComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId = boost::none;
auto task = api->unFlagComment(tenantId, commentId, userId, anonUserId)
    .then([](pplx::task<std::shared_ptr<FlagComment_200_response>> t){
        try {
            auto resp = t.get();
            auto finalResp = resp ? resp : std::make_shared<FlagComment_200_response>();
            (void)finalResp;
        } catch (const std::exception& e) {
            (void)e;
        }
    });
[inline-code-end]
