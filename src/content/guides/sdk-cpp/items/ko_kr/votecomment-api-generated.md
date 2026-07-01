## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| urlId | string | 예 |  |
| broadcastId | string | 예 |  |
| voteBodyParams | VoteBodyParams | 예 |  |
| options | const VoteCommentOptions& | 예 |  |

## 응답

반환: [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## 예제

[inline-code-attrs-start title = 'voteComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-7890");
auto urlId = utility::conversions::to_string_t("article-456");
auto broadcastId = utility::conversions::to_string_t("broadcast-321");

VoteBodyParams voteParams;
voteParams.upvote = true;
voteParams.note = boost::optional<utility::string_t>(utility::conversions::to_string_t("Great insight"));

VoteCommentOptions options;
options.dryRun = boost::optional<bool>(false);

api->voteComment(tenantId, commentId, urlId, broadcastId, voteParams, options)
   .then([](pplx::task<std::shared_ptr<VoteResponse>> t) {
       try {
           auto response = t.get();
           // 응답 처리
       } catch (const std::exception&) {
       }
   });
[inline-code-end]