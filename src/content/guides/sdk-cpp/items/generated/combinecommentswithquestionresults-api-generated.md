## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| questionId | string | No |  |
| questionIds | vector<string | No |  |
| urlId | string | No |  |
| startDate | datetime | No |  |
| forceRecalculate | bool | No |  |
| minValue | double | No |  |
| maxValue | double | No |  |
| limit | double | No |  |

## Response

Returns: [`CombineCommentsWithQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CombineCommentsWithQuestionResults_200_response.h)

## Example

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> questionId = boost::optional<utility::string_t>(U("q-456"));
boost::optional<std::vector<utility::string_t>> questionIds = std::vector<utility::string_t>{ U("q-456"), U("q-789") };
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("url-321"));
utility::datetime sd = utility::datetime::from_string(U("2025-01-01T00:00:00Z"));
boost::optional<utility::datetime> startDate = sd;
boost::optional<bool> forceRecalculate = true;
boost::optional<double> minValue = 0.0;
boost::optional<double> maxValue = 5.0;
boost::optional<double> limit = 100.0;
api->combineCommentsWithQuestionResults(tenantId, questionId, questionIds, urlId, startDate, forceRecalculate, minValue, maxValue, limit)
.then([](pplx::task<std::shared_ptr<CombineCommentsWithQuestionResults_200_response>> task){
    try {
        auto resp = task.get();
        auto copy = std::make_shared<CombineCommentsWithQuestionResults_200_response>(*resp);
        (void)copy;
    } catch(const std::exception &){
    }
});
[inline-code-end]
