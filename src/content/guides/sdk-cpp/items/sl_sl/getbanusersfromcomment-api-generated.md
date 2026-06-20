## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetBannedUsersFromCommentResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getBanUsersFromComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("comment-abc-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
std::shared_ptr<GetBannedUsersFromCommentResponse> result;
api->getBanUsersFromComment(commentId, sso).then([&result](pplx::task<std::shared_ptr<GetBannedUsersFromCommentResponse>> t){
    try {
        auto resp = t.get();
        if (resp) result = std::make_shared<GetBannedUsersFromCommentResponse>(*resp);
    } catch (const std::exception&) {
        result.reset();
    }
});
[inline-code-end]

---