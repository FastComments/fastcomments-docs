## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteEmailTemplate Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("tmpl-456");
boost::optional<utility::string_t> requestNote = boost::none;
api->deleteEmailTemplate(tenantId, templateId)
.then([=](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
    try {
        auto resp = t.get();
        auto safeResp = resp ? resp : std::make_shared<FlagCommentPublic_200_response>();
        (void)safeResp;
    } catch (const std::exception&) {
        throw;
    }
});
[inline-code-end]
