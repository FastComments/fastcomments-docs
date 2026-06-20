## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| broadcastId | string | Sì |  |
| editKey | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPIDeleteCommentResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteCommentPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---