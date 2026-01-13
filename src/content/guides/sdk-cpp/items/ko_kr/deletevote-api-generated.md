## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| editKey | string | 아니오 |  |

## 응답

반환: [`DeleteCommentVote_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentVote_200_response.h)

## 예제

[inline-code-attrs-start title = 'deleteVote 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t voteId = utility::conversions::to_string_t("vote-98765");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(utility::conversions::to_string_t("user-edit-key-abc123"));

api->deleteVote(tenantId, voteId, editKey)
.then([](pplx::task<std::shared_ptr<DeleteCommentVote_200_response>> t){
    try {
        auto resp = t.get();
        auto result = resp ? std::make_shared<DeleteCommentVote_200_response>(*resp)
                           : std::make_shared<DeleteCommentVote_200_response>();
        (void)result;
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]

---