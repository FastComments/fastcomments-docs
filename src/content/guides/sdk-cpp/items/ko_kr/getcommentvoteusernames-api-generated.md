## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| dir | int32_t | Yes |  |
| sso | string | No |  |

## 응답

반환: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNamesSuccessResponse.h)

## 예제

[inline-code-attrs-start title = 'getCommentVoteUserNames 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto task = api->getCommentVoteUserNames(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("comment-456"),
    static_cast<int32_t>(1),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("sso-token"))
).then([](pplx::task<std::shared_ptr<GetCommentVoteUserNamesSuccessResponse>> t){
    try{
        auto response = t.get();
    }catch(const std::exception&){ }
});
[inline-code-end]

---