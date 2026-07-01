## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример deleteNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto notificationId = utility::conversions::to_string_t("notif-789");

api->deleteNotificationCount(tenantId, notificationId)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {})
    .then([](pplx::task<void> t) { try { t.get(); } catch (const std::exception&) {} });
[inline-code-end]