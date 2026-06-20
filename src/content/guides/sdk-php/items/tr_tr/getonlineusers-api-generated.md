---
Bir sayfanın şu anda çevrimiçi olan izleyicileri: websocket oturumu şu anda sayfaya abone olan kişiler.
anonCount + totalCount döndürür (oda genelindeki aboneler, saymadığımız anonim izleyiciler dahil).

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıttan nextAfterName değerini verin. |
| afterUserId | string | query | No | İmleç eşitleyicisi: önceki yanıttan nextAfterUserId değerini verin. afterName ayarlandığında, isim eşitlikleri nedeniyle girdilerin atılmaması için gereklidir. |

## Yanıt

Döndürür: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Örnek

[inline-code-attrs-start title = 'getOnlineUsers Example'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi verin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
$after_name = 'after_name_example'; // string | İmleç: önceki yanıttan nextAfterName değerini verin.
$after_user_id = 'after_user_id_example'; // string | İmleç eşitleyicisi: önceki yanıttan nextAfterUserId değerini verin. afterName ayarlandığında, isim eşitlikleri nedeniyle girdilerin atılmaması için gereklidir.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---