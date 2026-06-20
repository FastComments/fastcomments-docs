## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | double | No |  |
| count | double | No |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| demo | bool | No |  |
| sso | string | No |  |

## Response

Returns: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetCommentsResponse.h)

## Example

[inline-code-attrs-start title = 'getApiComments Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<double> pageOpt(1.0);
boost::optional<double> countOpt(25.0);
boost::optional<utility::string_t> textSearchOpt(utility::conversions::to_string_t("offensive content"));
boost::optional<utility::string_t> byIPFromCommentOpt(utility::conversions::to_string_t("203.0.113.45"));
boost::optional<utility::string_t> filtersOpt(utility::conversions::to_string_t("{\"status\":\"pending\"}"));
boost::optional<utility::string_t> searchFiltersOpt(utility::conversions::to_string_t("author:john.doe@example.com"));
boost::optional<utility::string_t> sortsOpt(utility::conversions::to_string_t("createdAt:desc"));
boost::optional<bool> demoOpt(false);
boost::optional<utility::string_t> ssoOpt(utility::conversions::to_string_t("tenant-123"));

api->getApiComments(pageOpt, countOpt, textSearchOpt, byIPFromCommentOpt, filtersOpt, searchFiltersOpt, sortsOpt, demoOpt, ssoOpt)
.then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentsResponse>> t){
    try {
        auto resp = t.get();
        auto safeResp = resp ? resp : std::make_shared<ModerationAPIGetCommentsResponse>();
    } catch (const std::exception&) {}
})
.wait();
[inline-code-end]
