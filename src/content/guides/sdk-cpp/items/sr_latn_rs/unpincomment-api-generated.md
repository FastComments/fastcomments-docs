## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`PinComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PinComment_200_response.h)

## Primer

[inline-code-attrs-start title = 'Primer unPinComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
utility::string_t broadcastId = U("brd-98765");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto defaultResp = std::make_shared<PinComment_200_response>();
api->unPinComment(tenantId, commentId, broadcastId, sso)
.then([defaultResp](pplx::task<std::shared_ptr<PinComment_200_response>> task) {
    try {
        auto resp = task.get();
        if (!resp) resp = defaultResp;
        std::cout << "unPinComment completed; response pointer " << (resp ? "valid" : "null") << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "unPinComment failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---