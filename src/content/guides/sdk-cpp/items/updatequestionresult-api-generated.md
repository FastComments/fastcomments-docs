## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateQuestionResult Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("question-456");
auto updateQuestionResultBody = std::make_shared<UpdateQuestionResultBody>();
updateQuestionResultBody->result = U("accepted");
updateQuestionResultBody->updatedBy = U("moderator@example.com");
updateQuestionResultBody->notes = boost::optional<utility::string_t>(U("Confirmed by moderator"));
api->updateQuestionResult(tenantId, id, *updateQuestionResultBody)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task)
{
    try
    {
        auto resp = task.get();
        if (resp)
        {
            auto response_ptr = resp;
        }
    }
    catch (const std::exception &)
    {
    }
});
[inline-code-end]
