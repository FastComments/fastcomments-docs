## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | No |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Response

Returns: [`AddHashTagsBulk_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddHashTagsBulk_200_response.h)

## Example

[inline-code-attrs-start title = 'addHashTagsBulk Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
boost::optional<BulkCreateHashTagsBody> bulkBody = boost::optional<BulkCreateHashTagsBody>(); 
api->addHashTagsBulk(tenantId, bulkBody)
.then([](std::shared_ptr<AddHashTagsBulk_200_response> resp) {
    auto safeResp = resp ? resp : std::make_shared<AddHashTagsBulk_200_response>();
    return pplx::task_from_result(safeResp);
})
.then([](pplx::task<std::shared_ptr<AddHashTagsBulk_200_response>> t) {
    try {
        auto finalResp = t.get();
    } catch(...) {
    }
});
[inline-code-end]
