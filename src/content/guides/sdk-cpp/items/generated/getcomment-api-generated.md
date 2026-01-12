## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetComment_200_response.h)

## Example

[inline-code-attrs-start title = 'getComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-4e9a1b2");
boost::optional<utility::string_t> ifModifiedSince = U("2026-01-01T00:00:00Z");
api->getComment(tenantId, commentId)
    .then([ifModifiedSince](std::shared_ptr<GetComment_200_response> resp) {
        auto result = resp ? resp : std::make_shared<GetComment_200_response>();
        if(ifModifiedSince) { auto ims = *ifModifiedSince; (void)ims; }
        return result;
    });
[inline-code-end]
