---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | はい |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AdjustVotesResponse.h)

## 例

[inline-code-attrs-start title = 'postAdjustCommentVotes の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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