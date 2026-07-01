## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 是 |  |
| voteBodyParams | VoteBodyParams | 是 |  |
| options | const VoteCommentOptions& | 是 |  |

## 回應

返回：[`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## 範例

[inline-code-attrs-start title = 'voteComment 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
           // 處理回應
       } catch (const std::exception&) {
       }
   });
[inline-code-end]