## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tag | string | Так |  |
| tenantId | string | Ні |  |
| updateHashTagBody | UpdateHashTagBody | Ні |  |

## Відповідь

Повертає: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateHashTagResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад patchHashTag'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tag = U("bug");
boost::optional<utility::string_t> tenantId{ U("my-tenant-123") };
UpdateHashTagBody body;
boost::optional<UpdateHashTagBody> updateBody{ body };
api->patchHashTag(tag, tenantId, updateBody)
.then([](std::shared_ptr<UpdateHashTagResponse> resp)
{
    if (resp)
    {
        auto localCopy = std::make_shared<UpdateHashTagResponse>(*resp);
    }
});
[inline-code-end]

---