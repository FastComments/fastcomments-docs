## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nej |  |
| badgeId | string | Nej |  |
| type | double | Nej |  |
| displayedOnComments | bool | Nej |  |
| limit | double | Nej |  |
| skip | double | Nej |  |

## Svar

Returnerer: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgesResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getUserBadges Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> userId = utility::conversions::to_string_t("user@example.com");
boost::optional<utility::string_t> badgeId = utility::conversions::to_string_t("badge-elite-5");
boost::optional<double> type = 2.0;
boost::optional<bool> displayedOnComments = true;
boost::optional<double> limit = 50.0;
boost::optional<double> skip = 0.0;

api->getUserBadges(tenantId, userId, badgeId, type, displayedOnComments, limit, skip)
   .then([](pplx::task<std::shared_ptr<APIGetUserBadgesResponse>> t) {
       try {
           auto resp = t.get();
           auto copy = std::make_shared<APIGetUserBadgesResponse>(*resp);
           return copy;
       } catch (...) {
           return std::shared_ptr<APIGetUserBadgesResponse>();
       }
   });
[inline-code-end]

---