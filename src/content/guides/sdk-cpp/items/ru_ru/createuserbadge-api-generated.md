## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createUserBadgeParams | CreateUserBadgeParams | Да |  |

## Ответ

Возвращает: [`CreateUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateUserBadge_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример createUserBadge'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateUserBadgeParams params;
params.userId = U("user@example.com");
params.badgeId = U("trusted-contributor");
params.note = boost::optional<utility::string_t>(U("Awarded for outstanding moderation"));
api->createUserBadge(tenantId, params)
.then([](std::shared_ptr<CreateUserBadge_200_response> resp){
    if (resp) {
        auto copied = std::make_shared<CreateUserBadge_200_response>(*resp);
    }
})
.wait();
[inline-code-end]

---