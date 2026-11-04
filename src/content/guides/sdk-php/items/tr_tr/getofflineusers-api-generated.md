Şu anda çevrimiçi OLMAYAN, sayfadaki geçmiş yorumcular. displayName'e göre sıralanır.
"Üyeler" bölümünü görüntülemek için /users/online kaynağını tükettikten sonra bunu kullanın.
commenterName üzerinde imleç sayfalama: sunucu, {tenantId, urlId, commenterName} kısmi indeksini
afterName'den $gt ile ileriye doğru tarar, $skip maliyeti yoktur.

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | Hayır | İmleç: önceki yanıttan nextAfterName değerini iletin. |
| afterUserId | string | query | Hayır | İmleç eşitlik kırıcı: önceki yanıttan nextAfterUserId değerini iletin. İsim eşitliklerinde girdilerin düşmemesi için afterName ayarlandığında gereklidir. |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel http client kullanmak isterseniz, `GuzzleHttp\ClientInterface` uygulayan client'ınızı iletin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
$options = [
    'after_name' => 'after_name_example', // string | İmleç: önceki yanıttan nextAfterName değerini iletin.
    'after_user_id' => 'after_user_id_example', // string | İmleç eşitlik kırıcı: önceki yanıttan nextAfterUserId değerini iletin. İsim eşitliklerinde girdilerin düşmemesi için afterName ayarlandığında gereklidir.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]