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
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t urlId = utility::conversions::to_string_t("https://example.com/articles/42");
utility::string_t userId = utility::conversions::to_string_t("user@example.com");
utility::string_t parentId = utility::conversions::to_string_t("comment-abc-123");
api->getComments(tenantId,
                 boost::optional<int32_t>(1),
                 boost::optional<int32_t>(50),
                 boost::optional<int32_t>(0),
                 boost::optional<bool>(true),
                 boost::optional<int32_t>(),
                 boost::optional<int32_t>(),
                 boost::optional<int32_t>(),
                 boost::optional<utility::string_t>(urlId),
                 boost::optional<utility::string_t>(userId),
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(parentId),
                 boost::optional<SortDirections>())
.then([](std::shared_ptr<GetComments_200_response> resp){
    if (resp) {
        auto copy = std::make_shared<GetComments_200_response>(*resp);
    }
});
[inline-code-end]
