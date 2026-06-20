## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AdjustVotesResponse.h)

## 예제

[inline-code-attrs-start title = 'postAdjustCommentVotes 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
AdjustCommentVotesParams params;
params.userId = utility::string_t(U("user-742"));
params.adjustment = 1;
params.reason = utility::string_t(U("Marked as helpful"));

boost::optional<utility::string_t> sso = utility::string_t(U("sso-token-98765"));

api->postAdjustCommentVotes(utility::string_t(U("comment-5f3a9b2")), params, sso)
.then([](pplx::task<std::shared_ptr<AdjustVotesResponse>> t) {
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : std::make_shared<AdjustVotesResponse>();
        (void)finalResp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---