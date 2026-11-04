Bir tenant için toplu kullanıcı bilgisi. Verilen userIds değerlerine göre, User / SSOUser kaynaklarından görüntüleme bilgilerini döndürür.
Yorum widget'ı tarafından, bir varlık (presence) olayı aracılığıyla yeni görünen kullanıcıları zenginleştirmek için kullanılır.
Sayfa bağlamı yoktur: gizlilik tek tip şekilde uygulanır (özel profiller maskelenir).

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| ids | string | query | Evet | Virgülle ayrılmış userIds değerleri. |

## Yanıt

Döndürür: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Örnek

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir http client kullanmak isterseniz, `GuzzleHttp\ClientInterface` arayüzünü uygulayan client'ınızı geçirin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılır.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | Virgülle ayrılmış userIds değerleri.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]