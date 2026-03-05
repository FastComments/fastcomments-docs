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
boost::optional<int> pageLimit = 50;
boost::optional<utility::string_t> cursor = boost::none;
api->getPages(tenantId).then([tenantId](pplx::task<std::shared_ptr<GetPagesAPIResponse>> task) {
    try {
        auto resp = task.get();
        if(!resp) resp = std::make_shared<GetPagesAPIResponse>();
        std::wcout << U("Fetched pages for tenant: ") << tenantId << U("\n");
    } catch(const std::exception& e) {
        std::cerr << e.what() << std::endl;
    }
});
[inline-code-end]
