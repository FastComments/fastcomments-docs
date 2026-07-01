Sayfada daha önce yorum yapmış ancak şu anda çevrim içi olmayan yorumcular. displayName'e göre sıralanır.  
Bu, /users/online endpoint'i tüketildikten sonra bir "Üyeler" bölümü oluşturmak için kullanılır.  
commenterName üzerinde imleç sayfalama: sunucu, {tenantId, urlId, commenterName} kısmı üzerinden afterName'den itibaren $gt ile ileri yürür, $skip maliyeti yok.

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| options | const GetOfflineUsersOptions& | Evet |  |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---