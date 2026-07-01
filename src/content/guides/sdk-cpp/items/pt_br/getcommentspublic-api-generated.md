req
tenantId
urlId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| options | const GetCommentsPublicOptions& | Sim |  |

## Resposta

Retorna: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto urlId = U("article-456");
GetCommentsPublicOptions opts;
opts.pageSize = 20;
opts.includeDeleted = false;
api->getCommentsPublic(tenantId, urlId, opts).then([](std::shared_ptr<GetCommentsResponseWithPresence_PublicComment_> resp) {
    (void)resp;
});
[inline-code-end]

---