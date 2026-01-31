## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'createFeedPostPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateFeedPostParams createParams;
createParams.authorEmail = U("user@example.com");
createParams.authorName = U("Jane Doe");
createParams.title = U("New product release");
createParams.content = U("Announcing our latest update with improved performance and security.");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-9876"));
boost::optional<utility::string_t> sso = boost::none;
api->createFeedPostPublic(tenantId, createParams, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<CreateFeedPostPublic_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copy = std::make_shared<CreateFeedPostPublic_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]
