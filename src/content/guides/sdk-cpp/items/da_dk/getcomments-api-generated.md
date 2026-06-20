## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | int32_t | Nej |  |
| limit | int32_t | Nej |  |
| skip | int32_t | Nej |  |
| asTree | bool | Nej |  |
| skipChildren | int32_t | Nej |  |
| limitChildren | int32_t | Nej |  |
| maxTreeDepth | int32_t | Nej |  |
| urlId | string | Nej |  |
| userId | string | Nej |  |
| anonUserId | string | Nej |  |
| contextUserId | string | Nej |  |
| hashTag | string | Nej |  |
| parentId | string | Nej |  |
| direction | SortDirections | Nej |  |
| fromDate | int64_t | Nej |  |
| toDate | int64_t | Nej |  |

## Svar

Returnerer: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentsResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getComments Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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