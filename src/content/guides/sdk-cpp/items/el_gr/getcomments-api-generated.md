## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| page | int32_t | Όχι |  |
| limit | int32_t | Όχι |  |
| skip | int32_t | Όχι |  |
| asTree | bool | Όχι |  |
| skipChildren | int32_t | Όχι |  |
| limitChildren | int32_t | Όχι |  |
| maxTreeDepth | int32_t | Όχι |  |
| urlId | string | Όχι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |
| contextUserId | string | Όχι |  |
| hashTag | string | Όχι |  |
| parentId | string | Όχι |  |
| direction | SortDirections | Όχι |  |
| fromDate | int64_t | Όχι |  |
| toDate | int64_t | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentsResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getComments'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
boost::optional<int32_t> page = 1;
boost::optional<int32_t> limit = 50;
boost::optional<bool> asTree = true;
boost::optional<utility::string_t> userId = utility::string_t(U("user@example.com"));
boost::optional<int64_t> fromDate = 1622505600LL;
boost::optional<int64_t> toDate = 1625097600LL;

api->getComments(tenantId,
                 page,
                 limit,
                 boost::optional<int32_t>(),
                 asTree,
                 boost::optional<int32_t>(),
                 boost::optional<int32_t>(),
                 boost::optional<int32_t>(),
                 boost::optional<utility::string_t>(),
                 userId,
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<SortDirections>(),
                 fromDate,
                 toDate)
.then([](pplx::task<std::shared_ptr<APIGetCommentsResponse>> t){
    try {
        auto resp = t.get();
        auto holder = std::make_shared<std::shared_ptr<APIGetCommentsResponse>>(resp);
        (void)holder;
    } catch (...) {}
});
[inline-code-end]

---