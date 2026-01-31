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
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t commentId = utility::conversions::to_string_t("cmt-44827");
boost::optional<utility::string_t> includeReplies = boost::none;
api->getComment(tenantId, commentId)
.then([](pplx::task<std::shared_ptr<GetComment_200_response>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetComment_200_response>();
        return resp;
    } catch (...) {
        return std::make_shared<GetComment_200_response>();
    }
});
[inline-code-end]
