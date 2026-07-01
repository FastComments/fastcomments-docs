Passados comentadores na página que NÃO estão atualmente online. Ordenados por **displayName**.  
Use isso depois de esgotar `/users/online` para renderizar uma seção “Members”.  
Paginação por cursor em **commenterName**: o servidor percorre o parcial `{tenantId, urlId, commenterName}`  
índice a partir de **afterName** avançando via `$gt`, sem custo de `$skip`.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| options | const GetOfflineUsersOptions& | Sim |  |

## Response

Retorna: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Example

[inline-code-attrs-start title = 'Exemplo getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]