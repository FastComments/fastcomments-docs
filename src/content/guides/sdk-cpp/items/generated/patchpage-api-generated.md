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
utility::string_t pageId = U("page-987");
UpdateAPIPageData updateData;
updateData.title = boost::optional<utility::string_t>(U("Updated Comments Page"));
updateData.path = boost::optional<utility::string_t>(U("/products/widget-123/comments"));
updateData.isPublished = boost::optional<bool>(true);
api->patchPage(tenantId, pageId, updateData)
.then([](std::shared_ptr<PatchPageAPIResponse> resp) {
    if (resp) {
        auto updatedCopy = std::make_shared<PatchPageAPIResponse>(*resp);
    }
});
[inline-code-end]
