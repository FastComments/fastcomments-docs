## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteQuestionResult Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("question-456");
boost::optional<utility::string_t> moderatorNote = U("automated-moderation");
api->deleteQuestionResult(tenantId, id)
    .then([moderatorNote](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task) {
        try {
            auto result = task.get();
            if(!result) result = std::make_shared<FlagCommentPublic_200_response>();
            std::cout << "deleteQuestionResult completed, result present: " << (result ? "yes" : "no") << std::endl;
        } catch(const std::exception &e) {
            std::cerr << "deleteQuestionResult error: " << e.what() << std::endl;
        }
    });
[inline-code-end]
