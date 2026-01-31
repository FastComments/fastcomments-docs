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
utility::string_t tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> questionId = utility::string_t(U("q-456"));
boost::optional<std::vector<utility::string_t>> questionIds = std::vector<utility::string_t>{U("q-456"), U("q-789")};
boost::optional<utility::string_t> urlId = utility::string_t(U("/articles/how-to-cpp"));
boost::optional<utility::datetime> startDate = utility::datetime::from_string(utility::string_t(U("2025-01-01T00:00:00Z")));
boost::optional<bool> forceRecalculate = true;
boost::optional<double> minValue = 0.0;
boost::optional<double> maxValue = 1.0;
boost::optional<double> limit = 100.0;

api->combineCommentsWithQuestionResults(tenantId, questionId, questionIds, urlId, startDate, forceRecalculate, minValue, maxValue, limit)
.then([](pplx::task<std::shared_ptr<CombineCommentsWithQuestionResults_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto respCopy = std::make_shared<CombineCommentsWithQuestionResults_200_response>(*resp);
            std::cout << "CombineCommentsWithQuestionResults completed\n";
        }
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << '\n';
    }
});
[inline-code-end]
