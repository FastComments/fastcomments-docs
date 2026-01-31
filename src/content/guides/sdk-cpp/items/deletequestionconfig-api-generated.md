## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteQuestionConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
utility::string_t configId = U("question-config-456");
auto resultTask = api->deleteQuestionConfig(tenantId.value(), configId)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
        try {
            auto resp = t.get();
            return resp ? resp : std::make_shared<FlagCommentPublic_200_response>();
        } catch (...) {
            return std::make_shared<FlagCommentPublic_200_response>();
        }
    });
resultTask.wait();
[inline-code-end]
