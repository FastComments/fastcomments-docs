## Параметри

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| options | const DeleteSSOUserOptions& | Yes |  |

## Одговор

Враћа: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSSOUserAPIResponse.h)

## Пример

[inline-code-attrs-start title = 'Primer deleteSSOUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto userId = U("user-456");
DeleteSSOUserOptions options;
options.dryRun = boost::optional<bool>(true);
api->deleteSSOUser(tenantId, userId, options).then([](std::shared_ptr<DeleteSSOUserAPIResponse> resp) {
    if (resp) {
        (void)resp;
    }
});
[inline-code-end]

---