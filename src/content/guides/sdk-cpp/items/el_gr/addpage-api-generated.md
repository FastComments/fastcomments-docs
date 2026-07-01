## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createAPIPageData | CreateAPIPageData | Ναι |  |

## Response

Returns: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddPageAPIResponse.h)

## Example

[inline-code-attrs-start title = 'Παράδειγμα addPage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto createData = CreateAPIPageData{};
createData.title = utility::string_t(U("Welcome Page"));
createData.url = utility::string_t(U("https://example.com/welcome"));
createData.description = boost::optional<utility::string_t>(utility::string_t(U("Landing page for new users")));

api->addPage(utility::string_t(U("my-tenant-123")), createData)
    .then([](std::shared_ptr<AddPageAPIResponse> response) {
        if (response && response->success) {
            // διαχειριστείτε την επιτυχή προσθήκη
        } else {
            // διαχειριστείτε το σφάλμα
        }
    });
[inline-code-end]

---