## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPIPageData | UpdateAPIPageData | Yes |  |

## Response

Returns: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchPageAPIResponse.h)

## Example

[inline-code-attrs-start title = 'patchPage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t pageId = U("page-456");
UpdateAPIPageData updateData;
updateData.title = utility::string_t(U("Homepage - Updated"));
updateData.slug = boost::optional<utility::string_t>(U("homepage-updated"));
updateData.published = boost::optional<bool>(true);
auto fallbackResp = std::make_shared<PatchPageAPIResponse>();
api->patchPage(tenantId, pageId, updateData)
.then([](pplx::task<std::shared_ptr<PatchPageAPIResponse>> task)
{
    try
    {
        auto resp = task.get();
        if (resp)
            std::cout << "Patch succeeded" << std::endl;
        else
            std::cout << "Patch returned no data" << std::endl;
    }
    catch (const std::exception &e)
    {
        std::cerr << "Patch failed: " << e.what() << std::endl;
    }
});
[inline-code-end]
