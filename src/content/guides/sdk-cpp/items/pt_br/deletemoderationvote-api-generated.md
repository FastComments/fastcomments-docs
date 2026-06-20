## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| commentId | string | Sim |  |
| voteId | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteModerationVote'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("comment-98765");
utility::string_t voteId = U("vote-2468");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto task = api->deleteModerationVote(commentId, voteId, sso)
    .then([](pplx::task<std::shared_ptr<VoteDeleteResponse>> t){
        try {
            auto resp = t.get();
            if (resp) {
                auto confirmation = resp;
            } else {
                auto fallback = std::make_shared<VoteDeleteResponse>();
            }
        } catch (const std::exception& e) {
            auto errMsg = utility::conversions::to_string_t(e.what());
        }
    });
[inline-code-end]

---