## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| editKey | string | No |  |

## Response

Returns: [`DeleteCommentVote_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentVote_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteVote Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t voteId = utility::conversions::to_string_t("vote-7f3a9b");
boost::optional<utility::string_t> editKey = utility::conversions::to_string_t("edit-key-abc123");
api->deleteVote(tenantId, voteId, editKey)
    .then([](pplx::task<std::shared_ptr<DeleteCommentVote_200_response>> t) {
        try {
            auto resp = t.get();
            auto placeholder = std::make_shared<DeleteCommentVote_200_response>();
            (void)resp;
            (void)placeholder;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
