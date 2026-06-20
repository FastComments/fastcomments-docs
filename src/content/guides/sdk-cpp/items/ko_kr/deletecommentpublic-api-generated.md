## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 예 |  |
| editKey | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPIDeleteCommentResponse.h)

## 예제

[inline-code-attrs-start title = 'deleteCommentPublic 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t commentId = utility::conversions::to_string_t("cmt-456");
utility::string_t broadcastId = utility::conversions::to_string_t("video-789");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(utility::conversions::to_string_t("editkey-xyz"));
boost::optional<utility::string_t> sso = boost::none;
api->deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso)
.then([](pplx::task<std::shared_ptr<PublicAPIDeleteCommentResponse>> t){
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : std::make_shared<PublicAPIDeleteCommentResponse>();
        (void)finalResp;
    } catch (const std::exception&) {}
});
[inline-code-end]