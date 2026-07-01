## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| isFlagged | bool | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo flagCommentPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
bool isFlagged = true;
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("sso-token-abc");

api->flagCommentPublic(tenantId, commentId, isFlagged, sso)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
        try{
            auto resp = t.get();
        }catch(const std::exception&){
        }
    });
[inline-code-end]