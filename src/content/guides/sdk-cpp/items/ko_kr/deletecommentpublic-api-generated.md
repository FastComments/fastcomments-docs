## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| options | const DeleteCommentPublicOptions& | Yes |  |

## 응답

반환: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPIDeleteCommentResponse.h)

## 예시

[inline-code-attrs-start title = 'deleteCommentPublic 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("comment-456"));
auto broadcastId = utility::string_t(U("broadcast-789"));
DeleteCommentPublicOptions options;
options.force = boost::optional<bool>(true);
options.reason = boost::optional<utility::string_t>(U("Inappropriate content"));
api->deleteCommentPublic(tenantId, commentId, broadcastId, options)
    .then([](pplx::task<std::shared_ptr<PublicAPIDeleteCommentResponse>> t){
        try{
            auto resp = t.get();
        }catch(const std::exception& e){
        }
    });
[inline-code-end]