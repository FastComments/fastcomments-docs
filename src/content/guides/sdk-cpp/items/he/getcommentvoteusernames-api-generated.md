## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| dir | int32_t | כן |  |
| sso | string | לא |  |

## תשובה

מחזיר: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNamesSuccessResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentVoteUserNames'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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