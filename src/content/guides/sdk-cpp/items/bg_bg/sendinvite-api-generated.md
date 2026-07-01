## Parameters

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| fromName | string | Yes |  |

## Response

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Example

[inline-code-attrs-start title = 'Пример за sendInvite'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> cc = utility::conversions::to_string_t("cc@example.com");
api->sendInvite(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("invitee@example.com"),
    utility::conversions::to_string_t("John Doe")
).then([](std::shared_ptr<APIEmptyResponse> resp) {
    // обработи успешно поканата
});
[inline-code-end]