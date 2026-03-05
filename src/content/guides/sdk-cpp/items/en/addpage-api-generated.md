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
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
auto pageDataPtr = std::make_shared<CreateAPIPageData>();
pageDataPtr->url = utility::conversions::to_string_t("https://blog.example.com/posts/2026-product-release");
pageDataPtr->title = utility::conversions::to_string_t("2026 Product Release");
pageDataPtr->authorEmail = utility::conversions::to_string_t("author@example.com");
pageDataPtr->description = boost::optional<utility::string_t>(utility::conversions::to_string_t("Announcing our major 2026 release"));
pageDataPtr->isPublished = boost::optional<bool>(true);
api->addPage(tenantId, *pageDataPtr).then([](pplx::task<std::shared_ptr<AddPageAPIResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            std::cout << "addPage succeeded" << std::endl;
        } else {
            std::cout << "addPage returned null response" << std::endl;
        }
    } catch (const std::exception &e) {
        std::cerr << "addPage failed: " << e.what() << std::endl;
    }
});
[inline-code-end]
