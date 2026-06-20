## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## 응답

반환: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPIGetCommentTextResponse.h)

## 예제

[inline-code-attrs-start title = 'getCommentText 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-7f4b2a");
boost::optional<utility::string_t> editKey(utility::conversions::to_string_t("edit-xyz-789"));
boost::optional<utility::string_t> sso(utility::conversions::to_string_t("user@example.com"));
api->getCommentText(tenantId, commentId, editKey, sso)
.then([](pplx::task<std::shared_ptr<PublicAPIGetCommentTextResponse>> t){
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<PublicAPIGetCommentTextResponse>();
    } catch (...) {
        auto empty = std::make_shared<PublicAPIGetCommentTextResponse>();
    }
});
[inline-code-end]

---