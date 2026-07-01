Lister les pages d’un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salles.  
Nécessite `enableFChat` à **true** dans la configuration personnalisée résolue pour chaque page.  
Les pages qui nécessitent SSO sont filtrées en fonction des accès de groupe de l’utilisateur demandeur.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| options | const GetPagesPublicOptions& | Oui |  |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // process response if needed
    }catch(const std::exception&){
        // handle error if needed
    }
});
[inline-code-end]