## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Response

Returns: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPageByURLIdAPIResponse.h)

## Example

[inline-code-attrs-start title = 'getPageByURLId Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-7a9f-12345");
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(U("en-US"));
api->getPageByURLId(tenantId, urlId)
    .then([=](std::shared_ptr<GetPageByURLIdAPIResponse> response) {
        auto result = response ? response : std::make_shared<GetPageByURLIdAPIResponse>();
        return result;
    })
    .wait();
[inline-code-end]
