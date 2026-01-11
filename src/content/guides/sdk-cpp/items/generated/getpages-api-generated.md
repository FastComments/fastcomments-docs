## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPagesAPIResponse.h)

## Example

[inline-code-attrs-start title = 'getPages Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> pageToken = U("cursor_456");
auto pagesTask = api->getPages(tenantId).then([pageToken](pplx::task<std::shared_ptr<GetPagesAPIResponse>> t){
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetPagesAPIResponse>();
        return resp;
    } catch (...) {
        return std::make_shared<GetPagesAPIResponse>();
    }
});
[inline-code-end]
