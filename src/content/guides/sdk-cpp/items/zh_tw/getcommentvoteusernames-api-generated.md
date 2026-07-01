## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| dir | int32_t | 是 |  |
| sso | string | 否 |  |

## 回應

返回: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNamesSuccessResponse.h)

## 範例

[inline-code-attrs-start title = 'getCommentVoteUserNames 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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