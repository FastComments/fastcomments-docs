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
boost::optional<int32_t> page{1};
boost::optional<int32_t> limit{50};
boost::optional<bool> includeConfig{true};
boost::optional<bool> asTree{true};
boost::optional<int32_t> maxTreeDepth{3};
boost::optional<std::vector<utility::string_t>> hashTags{ std::vector<utility::string_t>{
    utility::conversions::to_string_t("support"), utility::conversions::to_string_t("release") } };
boost::optional<utility::string_t> searchText{ utility::conversions::to_string_t("login failed") };
boost::optional<utility::string_t> userId{ utility::conversions::to_string_t("user-789") };

api->getCommentsPublic(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("https://example.com/articles/456"),
    page, boost::optional<SortDirections>(), boost::optional<utility::string_t>(),
    boost::optional<int32_t>(), boost::optional<int32_t>(),
    limit, boost::optional<int32_t>(), boost::optional<bool>(),
    boost::optional<utility::string_t>(), includeConfig, boost::optional<bool>(),
    boost::optional<bool>(), utility::conversions::to_string_t("en-US"),
    boost::optional<utility::string_t>(), boost::optional<bool>(), boost::optional<bool>(),
    asTree, maxTreeDepth, boost::optional<bool>(), boost::optional<utility::string_t>(),
    searchText, hashTags, userId, boost::optional<utility::string_t>(),
    boost::optional<utility::string_t>(), boost::optional<utility::string_t>())
.then([](std::shared_ptr<GetCommentsPublic_200_response> resp){
    if (resp) std::cout << "Fetched comments payload\n";
});
[inline-code-end]
