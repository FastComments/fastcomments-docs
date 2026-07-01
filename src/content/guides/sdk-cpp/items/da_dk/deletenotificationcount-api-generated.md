## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Eksempel

[inline-code-attrs-start title = 'deleteNotificationCount Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto notificationId = utility::conversions::to_string_t("notif-789");

api->deleteNotificationCount(tenantId, notificationId)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {})
    .then([](pplx::task<void> t) { try { t.get(); } catch (const std::exception&) {} });
[inline-code-end]