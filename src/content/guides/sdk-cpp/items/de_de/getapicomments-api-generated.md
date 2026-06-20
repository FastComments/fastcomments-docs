## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| page | double | Nein |  |
| count | double | Nein |  |
| textSearch | string | Nein |  |
| byIPFromComment | string | Nein |  |
| filters | string | Nein |  |
| searchFilters | string | Nein |  |
| sorts | string | Nein |  |
| demo | bool | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetCommentsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getApiComments'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<double> pageOpt(1.0);
boost::optional<double> countOpt(25.0);
boost::optional<utility::string_t> textSearchOpt(utility::conversions::to_string_t("offensive content"));
boost::optional<utility::string_t> byIPFromCommentOpt(utility::conversions::to_string_t("203.0.113.45"));
boost::optional<utility::string_t> filtersOpt(utility::conversions::to_string_t("{\"status\":\"pending\"}"));
boost::optional<utility::string_t> searchFiltersOpt(utility::conversions::to_string_t("author:john.doe@example.com"));
boost::optional<utility::string_t> sortsOpt(utility::conversions::to_string_t("createdAt:desc"));
boost::optional<bool> demoOpt(false);
boost::optional<utility::string_t> ssoOpt(utility::conversions::to_string_t("tenant-123"));

api->getApiComments(pageOpt, countOpt, textSearchOpt, byIPFromCommentOpt, filtersOpt, searchFiltersOpt, sortsOpt, demoOpt, ssoOpt)
.then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentsResponse>> t){
    try {
        auto resp = t.get();
        auto safeResp = resp ? resp : std::make_shared<ModerationAPIGetCommentsResponse>();
    } catch (const std::exception&) {}
})
.wait();
[inline-code-end]