---
## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| editKey | string | No |  |

## Respons

Retourneert: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'deleteVote Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto voteId = utility::conversions::to_string_t("vote-9876");
boost::optional<utility::string_t> editKey = utility::conversions::to_string_t("edit-abc123");

api->deleteVote(tenantId, voteId, editKey).then([](pplx::task<std::shared_ptr<VoteDeleteResponse>> task) {
    try {
        auto response = task.get();
        // Verwerk de respons indien nodig
    } catch (const std::exception&) {
        // Fout afhandelen
    }
});
[inline-code-end]
---