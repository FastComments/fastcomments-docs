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
UpdateAPIPageData update;
update.title = boost::optional<utility::string_t>(U("Homepage - Updated"));
update.description = boost::optional<utility::string_t>(U("Updated landing page description"));
update.isEnabled = boost::optional<bool>(true);
api->patchPage(tenantId, pageId, update).then([](std::shared_ptr<PatchPageAPIResponse> resp){
    if(!resp) return std::make_shared<PatchPageAPIResponse>();
    return resp;
});
[inline-code-end]
