## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createAPISSOUserData | CreateAPISSOUserData | Да |  |

## Отговор

Връща: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddSSOUserAPIResponse.h)

## Пример

[inline-code-attrs-start title = 'addSSOUser Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
CreateAPISSOUserData createAPISSOUserData;
createAPISSOUserData.email = utility::conversions::to_string_t("john.doe@example.com");
createAPISSOUserData.externalId = utility::conversions::to_string_t("ext-9876");
createAPISSOUserData.firstName = boost::optional<utility::string_t>(utility::conversions::to_string_t("John"));
createAPISSOUserData.lastName = boost::optional<utility::string_t>(utility::conversions::to_string_t("Doe"));
api->addSSOUser(tenantId, createAPISSOUserData)
    .then([](std::shared_ptr<AddSSOUserAPIResponse> resp) {
    });
[inline-code-end]

---