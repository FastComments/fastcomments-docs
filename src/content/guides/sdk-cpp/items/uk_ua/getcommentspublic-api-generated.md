---
запит
tenantId
urlId

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|------------|------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| options | const GetCommentsPublicOptions& | Так |  |

## Відповідь

Повертає: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Приклад

[inline-code-attrs-start title = 'getCommentsPublic Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto urlId = U("article-456");
GetCommentsPublicOptions opts;
opts.pageSize = 20;
opts.includeDeleted = false;
api->getCommentsPublic(tenantId, urlId, opts).then([](std::shared_ptr<GetCommentsResponseWithPresence_PublicComment_> resp) {
    (void)resp;
});
[inline-code-end]