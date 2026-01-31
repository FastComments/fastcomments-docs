## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateQuestionConfig Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("qc-987654");
boost::optional<utility::string_t> moderatorEmail(U("moderator@myorg.com"));
UpdateQuestionConfigBody updateQuestionConfigBody(U("Is this comment inappropriate?"), true, moderatorEmail);
api->updateQuestionConfig(tenantId, id, updateQuestionConfigBody)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    auto result = resp ? resp : std::make_shared<FlagCommentPublic_200_response>();
    return result;
})
.wait();
[inline-code-end]
