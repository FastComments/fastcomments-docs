## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| isFlagged | bool | Yes |  |
| sso | string | No |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад flagCommentPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
bool isFlagged = true;
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("sso-token-abc");

api->flagCommentPublic(tenantId, commentId, isFlagged, sso)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
        try{
            auto resp = t.get();
        }catch(const std::exception&){
        }
    });
[inline-code-end]