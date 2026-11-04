Bir kiracı (tenant) için toplu kullanıcı bilgisi. Verilen userIds değerlerine göre, User / SSOUser kaynaklarından görüntüleme bilgilerini döndürür.
Yorum widget'ı tarafından, bir varlık (presence) olayı aracılığıyla yeni görünen kullanıcıları zenginleştirmek için kullanılır.
Sayfa bağlamı yoktur: gizlilik tek tip olarak uygulanır (özel profiller maskelenir).

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| ids | string | Evet |  |

## Yanıt

Döndürür: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Örnek

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // yanıtı işle
    }catch(const std::exception&){
        // hatayı yönet
    }
});
[inline-code-end]