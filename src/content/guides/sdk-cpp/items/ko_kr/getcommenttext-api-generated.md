## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| options | const GetCommentTextOptions& | 예 |  |

## 응답

반환: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPIGetCommentTextResponse.h)

## 예시

[inline-code-attrs-start title = 'getCommentText 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
auto options = std::make_shared<GetCommentTextOptions>();
options->language = boost::optional<utility::string_t>(U("en"));
options->includeDeleted = boost::optional<bool>(false);
api->getCommentText(tenantId, commentId, *options).then([](pplx::task<std::shared_ptr<PublicAPIGetCommentTextResponse>> task){
    try{
        auto response = task.get();
        (void)response;
    }catch(...){}
});
[inline-code-end]