## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| dir | int32_t | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNamesSuccessResponse.h)

## 範例

[inline-code-attrs-start title = 'getCommentVoteUserNames 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
int32_t dir = 1;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getCommentVoteUserNames(tenantId, commentId, dir, sso)
.then([](std::shared_ptr<GetCommentVoteUserNamesSuccessResponse> resp){
    auto result = resp ? resp : std::make_shared<GetCommentVoteUserNamesSuccessResponse>();
    std::cout << "Fetched comment vote user names" << std::endl;
});
[inline-code-end]

---