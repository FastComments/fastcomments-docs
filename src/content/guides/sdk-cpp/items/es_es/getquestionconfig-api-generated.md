## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigResponse.h)

## Example

[inline-code-attrs-start title = 'Ejemplo getQuestionConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto questionId = utility::conversions::to_string_t("question-456");

api->getQuestionConfig(tenantId, questionId)
    .then([](pplx::task<std::shared_ptr<GetQuestionConfigResponse>> task) {
        try {
            auto response = task.get();
            // Utilizar la respuesta según sea necesario
        } catch (const std::exception&) {
            // Manejar el error
        }
    });
[inline-code-end]