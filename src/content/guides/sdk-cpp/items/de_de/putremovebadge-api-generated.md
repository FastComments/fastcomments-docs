## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|---------------|--------------|
| tenantId | string | Ja |  |
| badgeId | string | Ja |  |
| options | const PutRemoveBadgeOptions& | Ja |  |

## Antwort

Rückgabe: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/RemoveUserBadgeResponse.h)

## Beispiel

[inline-code-attrs-start title = 'putRemoveBadge Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t badgeId = U("badge-abc-456");
PutRemoveBadgeOptions options;
options.reason = boost::optional<utility::string_t>(U("Spamming"));
api->putRemoveBadge(tenantId, badgeId, options).then([](std::shared_ptr<RemoveUserBadgeResponse> resp) {
    (void)resp;
});
[inline-code-end]