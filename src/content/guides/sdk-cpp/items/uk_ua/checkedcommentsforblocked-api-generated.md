## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentIds | string | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CheckBlockedCommentsResponse.h)

## Приклад

[inline-code-attrs-start title = 'checkedCommentsForBlocked Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto commentIds = U("cmt-001,cmt-002");
boost::optional<utility::string_t> sso = U("user@example.com");

api->checkedCommentsForBlocked(tenantId, commentIds, sno).then([](std::shared_ptr<CheckBlockedCommentsResponse> resp){
    (void)resp;
});
[inline-code-end]