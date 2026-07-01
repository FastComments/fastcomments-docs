req
tenantId
afterId

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| options | const GetFeedPostsPublicOptions& | Sí |  |

## Respuesta

Devuelve: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getFeedPostsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetFeedPostsPublicOptions{};
options.limit = boost::optional<int>{20};
options.before = boost::optional<utility::string_t>{U("2023-01-01T00:00:00Z")};

api->getFeedPostsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<PublicFeedPostsResponse>> task){
    try{
        auto response = task.get();
        auto processed = std::make_shared<PublicFeedPostsResponse>(*response);
        // Usar processed según sea necesario
    }catch(const std::exception&){
        // Manejar error
    }
});
[inline-code-end]

---