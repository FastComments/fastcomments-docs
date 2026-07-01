Şu anda çevrimiçi görüntüleyiciler: Websocket oturumu şu anda sayfaya abone olan kişiler.  
anonCount + totalCount değerini döndürür (odadaki tüm aboneler, saymadığımız anonim izleyiciler dahil).

## Parameters

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | Hayır | İmleç: bir önceki yanıtın nextAfterName değerini aktar. |
| afterUserId | string | query | Hayır | İmleç bağlayıcı: bir önceki yanıtın nextAfterUserId değerini aktar. afterName ayarlandığında, isim eşleşmelerinin girişleri atlamaması için gereklidir. |

## Response

Döndürür: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` arayüzünü uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır, `GuzzleHttp\Client` varsayılan olarak kullanılacaktır.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
$options = [
    'after_name' => 'after_name_example', // string | İmleç: bir önceki yanıtın nextAfterName değerini aktar.
    'after_user_id' => 'after_user_id_example', // string | İmleç bağlayıcı: bir önceki yanıtın nextAfterUserId değerini aktar. afterName ayarlandığında, isim eşleşmelerinin girişleri atlamaması için gereklidir.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---