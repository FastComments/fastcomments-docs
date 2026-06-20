## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| commentId | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de postRestoreDeletedComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("comment-8b3f4a2d");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->postRestoreDeletedComment(commentId, sso).then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
    try {
        std::shared_ptr<APIEmptyResponse> resp = t.get();
        if (!resp) resp = std::make_shared<APIEmptyResponse>();
    } catch (const std::exception&) {
    }
});
[inline-code-end]