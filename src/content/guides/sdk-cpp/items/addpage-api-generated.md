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
utility::string_t tenantId = U("my-tenant-123");
CreateAPIPageData pageData;
pageData.title = utility::string_t(U("Product Landing"));
pageData.url = utility::string_t(U("/product/home"));
pageData.authorEmail = utility::string_t(U("editor@example.com"));
pageData.published = true;
pageData.description = boost::optional<utility::string_t>(U("Landing page for the new product line"));
pageData.tags = std::make_shared<std::vector<utility::string_t>>(std::initializer_list<utility::string_t>{ U("marketing"), U("v2") });
api->addPage(tenantId, pageData).then([](pplx::task<std::shared_ptr<AddPageAPIResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            utility::string_t createdId = resp->pageId;
        }
    } catch (...) {}
});
[inline-code-end]
