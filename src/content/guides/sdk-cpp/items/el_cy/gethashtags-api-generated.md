## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| page | double | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetHashTags_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetHashTags_200_response.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getHashTags'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> page = 2.0;
auto defaultResp = std::make_shared<GetHashTags_200_response>();
api->getHashTags(tenantId, page).then([defaultResp](pplx::task<std::shared_ptr<GetHashTags_200_response>> t) {
    try {
        auto resp = t.get();
        if(!resp) resp = defaultResp;
        std::cout << "getHashTags completed; response object " << (resp ? "present" : "absent") << std::endl;
    } catch(const std::exception& e) {
        std::cout << "getHashTags error: " << e.what() << std::endl;
    }
});
[inline-code-end]

---