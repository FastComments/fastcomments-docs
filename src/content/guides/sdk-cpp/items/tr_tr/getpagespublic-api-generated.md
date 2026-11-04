Bir kiracı için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenen özel yapılandırmada `enableFChat` değerinin true olması gerekir.
SSO gerektiren sayfalar, istekte bulunan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| options | const GetPagesPublicOptions& | Evet |  |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Örnek

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // gerekirse yanıtı işle
    }catch(const std::exception&){
        // gerekirse hatayı ele al
    }
});
[inline-code-end]