## Παράμετροι

| Όνομα | Τύπος | Υποχρεωτικό | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| id | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetV2PageReactUsersResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getV2PageReactUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("https://www.example.com/articles/fastcomments-integration");
utility::string_t id = U("page-98765");
boost::optional<int> pageSize = boost::optional<int>(100);
pplx::task<std::shared_ptr<GetV2PageReactUsersResponse>> task =
    api->getV2PageReactUsers(tenantId, urlId, id)
    .then([](std::shared_ptr<GetV2PageReactUsersResponse> resp) {
        if (!resp) return std::shared_ptr<GetV2PageReactUsersResponse>();
        return std::make_shared<GetV2PageReactUsersResponse>(*resp);
    });
[inline-code-end]

---