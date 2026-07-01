## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| id | string | Ja |  |

## Response

Retourneert: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetV2PageReactUsersResponse.h)

## Example

[inline-code-attrs-start title = 'Voorbeeld getV2PageReactUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto urlId = utility::conversions::to_string_t("page-456");
auto id = utility::conversions::to_string_t("react-789");
boost::optional<utility::string_t> maybeFilter;
api->getV2PageReactUsers(tenantId, urlId, id).then([](pplx::task<std::shared_ptr<GetV2PageReactUsersResponse>> task){
    try{
        auto response = task.get();
        // Gebruik de response indien nodig
    }catch(const std::exception&){
        // Fout afhandelen
    }
});
[inline-code-end]