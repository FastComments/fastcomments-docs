## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| badgeId | string | Ja |  |
| options | const PutAwardBadgeOptions& | Ja |  |

## Svar

Returnerer: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AwardUserBadgeResponse.h)

## Eksempel

[inline-code-attrs-start title = 'putAwardBadge Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto badgeId = utility::conversions::to_string_t("badge-456");
PutAwardBadgeOptions opts;
opts.userId = utility::conversions::to_string_t("user-42");
opts.note = boost::optional<utility::string_t>(utility::conversions::to_string_t("Excellent comment"));
api->putAwardBadge(tenantId, badgeId, opts).then([](pplx::task<std::shared_ptr<AwardUserBadgeResponse>> t){
    try{
        auto resp = t.get();
        auto respCopy = std::make_shared<AwardUserBadgeResponse>(*resp);
    }catch(const std::exception& e){
    }
});
[inline-code-end]