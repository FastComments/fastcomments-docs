---
Bir kiracı için toplu kullanıcı bilgisi. Verilen userIds ile User / SSOUser tarafından görüntüleme bilgisi döndürülür.
Yorum widget'ı tarafından, presence event ile yeni görünmüş kullanıcıları zenginleştirmek için kullanılır.
Sayfa bağlamı yok: gizlilik tutarlı şekilde uygulanır (özel profiller maskelenir).

## Parametreler

| Ad | Tip | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| ids | string | Evet |  |

## Yanıt

Döndürür: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Örnek

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> statusFilter = U("active");
api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try {
        auto res = t.get();
        if (res) {
            auto responseCopy = std::make_shared<PageUsersInfoResponse>(*res);
        }
    } catch (const std::exception&) {}
});
[inline-code-end]

---