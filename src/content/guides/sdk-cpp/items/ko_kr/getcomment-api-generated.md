## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

Returns: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentResponse.h)

## 예시

[inline-code-attrs-start title = 'getComment 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-456");
boost::optional<int> maxDepth = boost::none;
api->getComment(tenantId, commentId).then([](pplx::task<std::shared_ptr<APIGetCommentResponse>> t){
    try{
        auto resp = t.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]