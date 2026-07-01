## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| editKey | string | Non |  |

## Réponse

Retourne : [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple deleteVote'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto voteId = utility::conversions::to_string_t("vote-9876");
boost::optional<utility::string_t> editKey = utility::conversions::to_string_t("edit-abc123");

api->deleteVote(tenantId, voteId, editKey).then([](pplx::task<std::shared_ptr<VoteDeleteResponse>> task) {
    try {
        auto response = task.get();
        // Traiter la réponse selon les besoins
    } catch (const std::exception&) {
        // Gérer l'erreur
    }
});
[inline-code-end]