## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| voteId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## 예제

[inline-code-attrs-start title = 'deleteModerationVote 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("comment-98765");
utility::string_t voteId = U("vote-2468");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto task = api->deleteModerationVote(commentId, voteId, sso)
    .then([](pplx::task<std::shared_ptr<VoteDeleteResponse>> t){
        try {
            auto resp = t.get();
            if (resp) {
                auto confirmation = resp;
            } else {
                auto fallback = std::make_shared<VoteDeleteResponse>();
            }
        } catch (const std::exception& e) {
            auto errMsg = utility::conversions::to_string_t(e.what());
        }
    });
[inline-code-end]