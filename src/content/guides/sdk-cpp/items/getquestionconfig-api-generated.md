## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetQuestionConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfig_200_response.h)

## Example

[inline-code-attrs-start title = 'getQuestionConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t questionId = utility::conversions::to_string_t("q-456");
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(utility::conversions::to_string_t("en-US"));
auto task = api->getQuestionConfig(tenantId, questionId).then([](pplx::task<std::shared_ptr<GetQuestionConfig_200_response>> t){
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetQuestionConfig_200_response>();
        (void)resp;
    } catch (const std::exception&) {
    }
});
task.wait();
[inline-code-end]
