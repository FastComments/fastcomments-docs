Sayfada daha önce yorum yapan ve şu anda çevrimiçi olmayan kişiler. displayName'e göre sıralanır.
Bir "Üyeler" bölümü görüntülemek için /users/online tükendikten sonra bunu kullanın.
commenterName üzerinde imleç sayfalandırması: sunucu kısmi {tenantId, urlId, commenterName} dizinini afterName'den ileriye doğru $gt ile tarar, $skip maliyeti yok.

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıttan nextAfterName değerini aktarın. |
| afterUserId | string | query | No | İmleç eşitleyicisi: önceki yanıttan nextAfterUserId değerini aktarın. afterName ayarlandığında, aynı ada sahip kayıtların düşmemesi için gereklidir. |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
$after_name = 'after_name_example'; // string | İmleç: önceki yanıttan nextAfterName değerini aktarın.
$after_user_id = 'after_user_id_example'; // string | İmleç eşitleyicisi: önceki yanıttan nextAfterUserId değerini aktarın. afterName ayarlandığında, aynı ada sahip kayıtların düşmemesi için gereklidir.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]