## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| dir | int32_t | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNamesSuccessResponse.h)

## 例

[inline-code-attrs-start title = 'getCommentVoteUserNames の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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