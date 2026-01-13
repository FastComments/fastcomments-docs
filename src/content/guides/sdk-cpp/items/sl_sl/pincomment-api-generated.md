## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`PinComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PinComment_200_response.h)

## Primer

[inline-code-attrs-start title = 'Primer pinComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
utility::string_t broadcastId = U("broadcast-98765");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->pinComment(tenantId, commentId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<PinComment_200_response>> t){
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<PinComment_200_response>();
        (void)resp;
    } catch (...) {}
}).wait();
[inline-code-end]