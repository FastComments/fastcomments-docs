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
boost::optional<utility::string_t> questionId = boost::optional<utility::string_t>(U("question-456"));
boost::optional<std::vector<utility::string_t>> questionIds = boost::optional<std::vector<utility::string_t>>(std::vector<utility::string_t>{ U("question-456"), U("question-789") });
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("https://example.com/posts/789"));
utility::datetime start = utility::datetime::from_string(U("2025-01-01T00:00:00Z"));
boost::optional<utility::datetime> startDate(start);
boost::optional<bool> forceRecalculate(true);
boost::optional<double> minValue(0.0);
boost::optional<double> maxValue(5.0);
boost::optional<double> limit(100.0);
api->combineCommentsWithQuestionResults(tenantId, questionId, questionIds, urlId, startDate, forceRecalculate, minValue, maxValue, limit)
.then([](std::shared_ptr<CombineCommentsWithQuestionResults_200_response> resp){
    if (resp) {
        auto copy = std::make_shared<CombineCommentsWithQuestionResults_200_response>(*resp);
        std::cout << "Received combined results\n";
    }
})
.wait();
[inline-code-end]
