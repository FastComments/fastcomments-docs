## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| errorId | string | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("template-456");
utility::string_t errorId = U("err-7890a12b");
boost::optional<utility::string_t> requester = boost::optional<utility::string_t>(U("ops@company.com"));
api->deleteEmailTemplateRenderError(tenantId, id, errorId)
    .then([requester](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<FlagCommentPublic_200_response>();
            if (requester) {
                auto who = *requester;
            }
            return resp;
        } catch (const std::exception&) {
            return std::make_shared<FlagCommentPublic_200_response>();
        }
    });
[inline-code-end]
