## 參數

| 名稱 | 類型 | 必須 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| options | const PostFlagCommentOptions& | 是 |  |

## 回應

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'postFlagComment 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
PostFlagCommentOptions options;
options.reason = boost::optional<utility::string_t>(U("spam"));
options.reportedBy = boost::optional<utility::string_t>(U("moderator@example.com"));

api->postFlagComment(U("my-tenant-123"), U("comment-456"), options)
    .then([](std::shared_ptr<APIEmptyResponse> resp){
        (void)resp;
    })
    .then([](pplx::task<void> t){
        try{
            t.get();
        }catch(const std::exception&){
        }
    });
[inline-code-end]