## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| page | int32_t | No |  |
| direction | SortDirections | No |  |
| sso | string | No |  |
| skip | int32_t | No |  |
| skipChildren | int32_t | No |  |
| limit | int32_t | No |  |
| limitChildren | int32_t | No |  |
| countChildren | bool | No |  |
| fetchPageForCommentId | string | No |  |
| includeConfig | bool | No |  |
| countAll | bool | No |  |
| includei10n | bool | No |  |
| locale | string | No |  |
| modules | string | No |  |
| isCrawler | bool | No |  |
| includeNotificationCount | bool | No |  |
| asTree | bool | No |  |
| maxTreeDepth | int32_t | No |  |
| useFullTranslationIds | bool | No |  |
| parentId | string | No |  |
| searchText | string | No |  |
| hashTags | vector<string | No |  |
| userId | string | No |  |
| customConfigStr | string | No |  |
| afterCommentId | string | No |  |
| beforeCommentId | string | No |  |

## Response

Returns: [`GetCommentsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'getCommentsPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->getCommentsPublic(
    utility::string_t(U("my-tenant-123")),
    utility::string_t(U("/articles/how-to-cpp-async")),
    boost::optional<int32_t>(1),
    boost::optional<SortDirections>(SortDirections::DESC)
).then([](pplx::task<std::shared_ptr<GetCommentsPublic_200_response>> t){
    try {
        auto resp = t.get();
        auto copy = std::make_shared<GetCommentsPublic_200_response>(*resp);
    } catch (...) {}
});
[inline-code-end]
