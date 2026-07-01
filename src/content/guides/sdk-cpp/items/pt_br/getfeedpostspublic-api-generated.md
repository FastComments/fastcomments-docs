req  
tenantId  
afterId  

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| options | const GetFeedPostsPublicOptions& | Sim |  |

## Resposta

Retorna: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getFeedPostsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
auto options = GetFeedPostsPublicOptions{};  
options.limit = boost::optional<int>{20};  
options.before = boost::optional<utility::string_t>{U("2023-01-01T00:00:00Z")};  

api->getFeedPostsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<PublicFeedPostsResponse>> task){  
    try{  
        auto response = task.get();  
        auto processed = std::make_shared<PublicFeedPostsResponse>(*response);  
        // Use o processado conforme necessário  
    }catch(const std::exception&){  
        // Trate o erro  
    }  
});  
[inline-code-end]  

---