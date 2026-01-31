## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | int32_t | No |  |
| limit | int32_t | No |  |
| skip | int32_t | No |  |
| asTree | bool | No |  |
| skipChildren | int32_t | No |  |
| limitChildren | int32_t | No |  |
| maxTreeDepth | int32_t | No |  |
| urlId | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |
| contextUserId | string | No |  |
| hashTag | string | No |  |
| parentId | string | No |  |
| direction | SortDirections | No |  |

## Response

Returns: [`GetComments_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetComments_200_response.h)

## Example

[inline-code-attrs-start title = 'getComments Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<int32_t> page = 1;
boost::optional<int32_t> limit = 50;
boost::optional<int32_t> skip = 0;
boost::optional<bool> asTree = true;
boost::optional<int32_t> skipChildren = 0;
boost::optional<int32_t> limitChildren = 10;
boost::optional<int32_t> maxTreeDepth = 3;
boost::optional<utility::string_t> urlId = U("article-42");
boost::optional<utility::string_t> userId = U("user-987");
boost::optional<utility::string_t> anonUserId = U("anon-abc-123");
boost::optional<utility::string_t> contextUserId = U("context-user-555");
boost::optional<utility::string_t> hashTag = U("release");
boost::optional<utility::string_t> parentId = U("comment-12");
boost::optional<SortDirections> direction = SortDirections::DESC;
api->getComments(tenantId, page, limit, skip, asTree, skipChildren, limitChildren, maxTreeDepth, urlId, userId, anonUserId, contextUserId, hashTag, parentId, direction)
.then([](pplx::task<std::shared_ptr<GetComments_200_response>> t){
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<GetComments_200_response>();
        (void)result;
    } catch(const std::exception&) {
    }
});
[inline-code-end]
