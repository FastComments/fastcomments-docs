## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| search | string | Yes |  |
| options | const GetGifsSearchOptions& | Yes |  |

## Réponse

Renvoie : [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetGifsSearchResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getGifsSearch'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto search = U("funny cats");
GetGifsSearchOptions options;
options.limit = boost::optional<int>(10);
options.rating = boost::optional<utility::string_t>(U("pg"));
api->getGifsSearch(tenantId, search, options).then([](pplx::task<std::shared_ptr<GetGifsSearchResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]