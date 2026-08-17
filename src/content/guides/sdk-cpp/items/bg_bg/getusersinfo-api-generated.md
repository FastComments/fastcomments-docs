---
Групова информация за потребители за наемател. При зададени userIds, връща информация за показване от User / SSOUser.  
Използва се от уиджета за коментари, за да обогати потребителите, които току‑що се появяват чрез събитие за присъствие.  
Без контекст на страницата: поверителността се прилага еднородно (частните профили се маскират).

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Отговор

Връща: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // обработка на отговора
    }catch(const std::exception&){
        // обработка на грешка
    }
});
[inline-code-end]

---