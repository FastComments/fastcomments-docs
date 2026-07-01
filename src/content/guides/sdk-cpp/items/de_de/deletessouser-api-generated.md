## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| options | const DeleteSSOUserOptions& | Ja |  |

## Antwort

Rückgabe: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSSOUserAPIResponse.h)

## Beispiel

[inline-code-attrs-start title = 'deleteSSOUser Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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