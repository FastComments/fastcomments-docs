## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Ναι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentBanStatusResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentBanStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso = utility::string_t(U("user@example.com"));
utility::string_t commentId = utility::string_t(U("comment-98765"));

api->getCommentBanStatus(commentId, sso)
    .then([](pplx::task<std::shared_ptr<GetCommentBanStatusResponse>> task){
        try {
            auto resp = task.get();
            auto copy = std::make_shared<GetCommentBanStatusResponse>(*resp);
            (void)copy;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---