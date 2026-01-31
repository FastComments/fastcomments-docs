## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | Yes |  |
| tenantId | string | No |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Response

Returns: [`PatchHashTag_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchHashTag_200_response.h)

## Example

[inline-code-attrs-start title = 'patchHashTag Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tag = utility::string_t(U("feature-requests"));
boost::optional<utility::string_t> tenantId(utility::string_t(U("my-tenant-123")));
UpdateHashTagBody body;
body.name = utility::string_t(U("Feature Requests"));
body.description = utility::string_t(U("Requests for new product features"));
boost::optional<UpdateHashTagBody> updateHashTagBody(body);
api->patchHashTag(tag, tenantId, updateHashTagBody)
.then([](pplx::task<std::shared_ptr<PatchHashTag_200_response>> task)
{
    try {
        auto resp = task.get();
        auto result = std::make_shared<PatchHashTag_200_response>();
        if (resp) *result = *resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]
