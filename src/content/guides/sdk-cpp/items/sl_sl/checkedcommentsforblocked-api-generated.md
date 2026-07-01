## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentIds | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CheckBlockedCommentsResponse.h)

## Primer

[inline-code-attrs-start title = 'checkedCommentsForBlocked Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto commentIds = U("cmt-001,cmt-002");
boost::optional<utility::string_t> sso = U("user@example.com");

api->checkedCommentsForBlocked(tenantId, commentIds, sso).then([](std::shared_ptr<CheckBlockedCommentsResponse> resp){
    (void)resp;
});
[inline-code-end]