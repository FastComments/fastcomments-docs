## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeletePageAPIResponse.h)

## Example

[inline-code-attrs-start title = 'deletePage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t pageId = U("page-456");
boost::optional<utility::string_t> auditNote = boost::optional<utility::string_t>(U("cleanup-unused-page"));
auto task = api->deletePage(tenantId, pageId).then([=](std::shared_ptr<DeletePageAPIResponse> resp) {
    return resp ? resp : std::make_shared<DeletePageAPIResponse>();
});
auto finalResp = task.get();
[inline-code-end]
