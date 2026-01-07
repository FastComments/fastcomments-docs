## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIPageData | CreateAPIPageData | Yes |  |

## Response

Returns: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddPageAPIResponse.h)

## Example

[inline-code-attrs-start title = 'addPage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto pageData = std::make_shared<CreateAPIPageData>();
pageData->url = utility::string_t(U("https://example.com/articles/2025-modern-cpp"));
pageData->title = utility::string_t(U("Modern C++ Patterns"));
pageData->description = boost::optional<utility::string_t>(utility::string_t(U("Practical patterns for modern C++ development")));
pageData->authorEmail = boost::optional<utility::string_t>(utility::string_t(U("dev@company.com")));
pageData->published = boost::optional<bool>(true);

api->addPage(utility::string_t(U("my-tenant-123")), *pageData)
.then([](pplx::task<std::shared_ptr<AddPageAPIResponse>> t){
    try {
        auto resp = t.get();
        return resp;
    } catch (...) {
        throw;
    }
});
[inline-code-end]
