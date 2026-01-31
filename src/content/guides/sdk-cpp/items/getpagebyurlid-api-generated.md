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
boost::optional<utility::string_t> tenantOpt(U("my-tenant-123"));
utility::string_t tenantId = tenantOpt ? *tenantOpt : U("my-tenant-123");
utility::string_t urlId = U("9f8b7c6d-1234-5678-90ab-abcdef123456");
auto getPageTask = api->getPageByURLId(tenantId, urlId)
.then([](pplx::task<std::shared_ptr<GetPageByURLIdAPIResponse>> previous) {
    try {
        auto resp = previous.get();
        auto result = resp ? resp : std::make_shared<GetPageByURLIdAPIResponse>();
        return result;
    } catch (const std::exception&) {
        return std::make_shared<GetPageByURLIdAPIResponse>();
    }
});
[inline-code-end]
