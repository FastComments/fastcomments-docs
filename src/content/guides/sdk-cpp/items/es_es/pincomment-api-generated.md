## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeCommentPinStatusResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de pinComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
utility::string_t broadcastId = U("broadcast-987");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->pinComment(tenantId, commentId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<ChangeCommentPinStatusResponse>> task){
    try {
        auto resp = task.get();
        auto copy = std::make_shared<ChangeCommentPinStatusResponse>(*resp);
        std::cout << "Pin operation completed. Response present: " << (resp != nullptr) << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Pin failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---