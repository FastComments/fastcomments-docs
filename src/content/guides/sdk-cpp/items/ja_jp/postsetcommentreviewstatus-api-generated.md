## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostSetCommentReviewStatusOptions& | Yes |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'postSetCommentReviewStatus の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
PostSetCommentReviewStatusOptions opts;
opts.status = utility::conversions::to_string_t("approved");
opts.note = boost::optional<utility::string_t>(utility::conversions::to_string_t("Looks good"));
api->postSetCommentReviewStatus(tenantId, commentId, opts)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
        try{
            auto resp = t.get();
        }catch(const std::exception& e){
        }
    });
[inline-code-end]