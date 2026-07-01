## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| tag | string | Sim |  |
| updateHashTagBody | UpdateHashTagBody | Sim |  |

## Resposta

Retorna: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateHashTagResponse.h)

## Exemplo

[inline-code-attrs-start title = 'patchHashTag Exemplo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t tag = U("important");
auto body = std::make_shared<UpdateHashTagBody>();
body->name = U("important-updated");
body->description = boost::optional<utility::string_t>(U("Updated description"));
api->patchHashTag(tenantId, tag, *body)
   .then([](pplx::task<std::shared_ptr<UpdateHashTagResponse>> t){
       try{
           auto response = t.get();
           std::cout << "Updated tag ID: " << response->id << std::endl;
       }catch(const std::exception& e){
           std::cerr << "Patch failed: " << e.what() << std::endl;
       }
   });
[inline-code-end]