Informations utilisateur en masse pour un tenant. Étant donné des userIds, renvoie les informations d'affichage de User / SSOUser.
Utilisé par le widget de commentaires pour enrichir les utilisateurs qui viennent d'apparaître via un événement de présence.
Pas de contexte de page : la confidentialité est appliquée de manière uniforme (les profils privés sont masqués).

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| ids | string | Oui |  |

## Réponse

Renvoie : [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // traiter la réponse
    }catch(const std::exception&){
        // gérer l'erreur
    }
});
[inline-code-end]