## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | Sim |  |
| options | const PostAdjustCommentVotesOptions& | Sim |  |

## Resposta

Retorna: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AdjustVotesResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo postAdjustCommentVotes'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
AdjustCommentVotesParams params;
params.voteDelta = 1;
params.userIdentifier = utility::conversions::to_string_t("user@example.com");
params.reason = boost::optional<utility::string_t>(utility::conversions::to_string_t("Helpful"));
PostAdjustCommentVotesOptions opts;
opts.timeout = boost::optional<int>(30);
api->postAdjustCommentVotes(tenantId, commentId, params, opts).then([](pplx::task<std::shared_ptr<AdjustVotesResponse>> t){
    try{
        auto resp = t.get();
    }catch(const std::exception& e){
    }
});
[inline-code-end]