## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | string | Sim |  |
| isFlagged | bool | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de flagCommentPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
bool isFlagged = true;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->flagCommentPublic(tenantId, commentId, isFlagged, sso)
    .then([](std::shared_ptr<APIEmptyResponse> resp){
        auto result = resp ? resp : std::make_shared<APIEmptyResponse>();
        std::cout << "Flag update completed\n";
    });
[inline-code-end]

---