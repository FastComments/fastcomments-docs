## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | No |  |
| userId | string | No |  |
| startDate | string | No |  |
| questionId | string | No |  |
| questionIds | string | No |  |
| skip | double | No |  |

## Response

Returns: [`GetQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResults_200_response.h)

## Example

[inline-code-attrs-start title = 'getQuestionResults Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> urlId = U("landing-page-456");
boost::optional<utility::string_t> userId = U("user@example.com");
boost::optional<utility::string_t> startDate = U("2024-01-01T00:00:00Z");
boost::optional<utility::string_t> questionId = U("q-789");
boost::optional<utility::string_t> questionIds = U("q-789,q-790");
boost::optional<double> skip = 0.0;

api->getQuestionResults(tenantId, urlId, userId, startDate, questionId, questionIds, skip)
.then([](std::shared_ptr<GetQuestionResults_200_response> resp){
    auto resultCopy = std::make_shared<GetQuestionResults_200_response>(*resp);
    (void)resultCopy;
});
[inline-code-end]
