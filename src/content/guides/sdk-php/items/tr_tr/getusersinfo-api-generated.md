Toplu kullanıcı bilgisi bir tenant için. Verilen userIds'e göre, User / SSOUser'dan görüntüleme bilgilerini döndürür.
Yorum widget'ı tarafından, presence olayıyla yeni görünen kullanıcıları zenginleştirmek için kullanılır.
Sayfa bağlamı yok: gizlilik eşit şekilde uygulanır (özel profiller maskelenir).

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| ids | string | query | Evet | Virgülle ayrılmış userIds. |

## Yanıt

Döndürür: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Örnek

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir http istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, `GuzzleHttp\Client` varsayılan olarak kullanılacaktır.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | Virgülle ayrılmış userIds.

try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]