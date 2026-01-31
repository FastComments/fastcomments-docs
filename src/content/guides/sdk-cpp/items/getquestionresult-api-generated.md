## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetQuestionResult_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResult_200_response.h)

## Example

[inline-code-attrs-start title = 'getQuestionResult Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
utility::string_t id = utility::string_t("qst-789");
boost::optional<utility::string_t> correlationId = boost::optional<utility::string_t>(utility::string_t("corr-001"));
auto task = api->getQuestionResult(tenantId, id)
.then([=](pplx::task<std::shared_ptr<GetQuestionResult_200_response>> prev)->void {
    try {
        auto resp = prev.get();
        if (resp) {
            auto resultCopy = std::make_shared<GetQuestionResult_200_response>(*resp);
        } else {
            auto emptyResult = std::make_shared<GetQuestionResult_200_response>();
        }
    } catch (const std::exception&) {
        auto errorResult = std::make_shared<GetQuestionResult_200_response>();
    }
});
[inline-code-end]
