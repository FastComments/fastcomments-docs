## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |

## Ответ

Возвращает: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPagesAPIResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getPages'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
int main()
{
    utility::string_t tenantId = U("my-tenant-123");
    boost::optional<utility::string_t> statusFilter = U("published");
    boost::optional<int> pageNumber = 1;
    auto placeholder = std::make_shared<GetPagesAPIResponse>();
    api->getPages(tenantId)
       .then([placeholder](std::shared_ptr<GetPagesAPIResponse> resp) {
           if (resp) {
               *placeholder = *resp;
               auto refs = resp.use_count();
               (void)refs;
           }
       })
       .wait();
    return 0;
}
[inline-code-end]