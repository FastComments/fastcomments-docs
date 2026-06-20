## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteEmailTemplate'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email-001");
boost::optional<utility::string_t> auditReason = boost::none;
api->deleteEmailTemplate(tenantId, templateId).then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try {
        auto resp = task.get();
        if(!resp) {
            auto fallback = std::make_shared<APIEmptyResponse>();
        }
    } catch(const std::exception&) {
    }
});
[inline-code-end]

---