## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| value | string | Non |  |
| filters | string | Non |  |
| searchFilters | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationCommentSearchResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getSearchCommentsSummary'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> value = boost::optional<utility::string_t>(U("hate speech"));
boost::optional<utility::string_t> filters = boost::optional<utility::string_t>(U("tenantId:my-tenant-123;moderationStatus:unreviewed"));
boost::optional<utility::string_t> searchFilters = boost::optional<utility::string_t>(U("authorEmail:moderator@example.com"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("my-sso-jwt-token-abc123"));
api->getSearchCommentsSummary(value, filters, searchFilters, sso)
    .then([](std::shared_ptr<ModerationCommentSearchResponse> resp){
        auto result = resp ? resp : std::make_shared<ModerationCommentSearchResponse>();
        (void)result;
    });
[inline-code-end]

---