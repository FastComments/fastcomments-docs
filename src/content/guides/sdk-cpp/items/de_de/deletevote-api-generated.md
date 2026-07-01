## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| editKey | string | Nein |  |

## Antwort

Rückgabe: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## Beispiel

[inline-code-attrs-start title = 'deleteVote Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto voteId = utility::conversions::to_string_t("vote-9876");
boost::optional<utility::string_t> editKey = utility::conversions::to_string_t("edit-abc123");

api->deleteVote(tenantId, voteId, editKey).then([](pplx::task<std::shared_ptr<VoteDeleteResponse>> task) {
    try {
        auto response = task.get();
        // Antwort verarbeiten nach Bedarf
    } catch (const std::exception&) {
        // Fehler behandeln
    }
});
[inline-code-end]