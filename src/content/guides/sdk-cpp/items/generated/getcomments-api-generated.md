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
auto tenantId = utility::string_t(U("my-tenant-123"));
auto page = boost::optional<int32_t>(1);
auto limit = boost::optional<int32_t>(25);
auto asTree = boost::optional<bool>(true);
auto maxDepth = boost::optional<int32_t>(3);
auto urlId = boost::optional<utility::string_t>(utility::string_t(U("/articles/2025/cpp-async")));
auto userId = boost::optional<utility::string_t>(utility::string_t(U("user@example.com")));
auto contextUserId = boost::optional<utility::string_t>(utility::string_t(U("ctx-user-42")));
auto direction = boost::optional<SortDirections>(SortDirections::DESC);

api->getComments(
    tenantId,
    page,
    limit,
    boost::optional<int32_t>(), 
    asTree,
    boost::optional<int32_t>(),
    boost::optional<int32_t>(),
    maxDepth,
    urlId,
    userId,
    boost::optional<utility::string_t>(),
    contextUserId,
    boost::optional<utility::string_t>(),
    boost::optional<utility::string_t>(),
    direction
).then([](std::shared_ptr<GetComments_200_response> resp){
    if (!resp) return;
    auto copy = std::make_shared<GetComments_200_response>(*resp);
});
[inline-code-end]
