Bir kiracı için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenen özel yapılandırmada `enableFChat` değerinin true olması gerekir.
SSO gerektiren sayfalar, istekte bulunan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| cursor | string | query | Hayır | Önceki bir istekten `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` değerine bağlıdır. |
| limit | integer | query | Hayır | 1..200, varsayılan 50 |
| q | string | query | Hayır | İsteğe bağlı, büyük/küçük harfe duyarsız başlık öneki filtresi. |
| sortBy | string | query | Hayır | Sıralama düzeni. `updatedAt` (varsayılan, en yeniden eskiye), `commentCount` (en çok yorumlu önce) veya `title` (alfabetik). |
| hasComments | boolean | query | Hayır | true ise, yalnızca en az bir yorumu olan sayfaları döndürür. |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Örnek

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir http istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Önceki bir istekten `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` değerine bağlıdır.
    'limit' => 56, // int | 1..200, varsayılan 50
    'q' => 'q_example', // string | İsteğe bağlı, büyük/küçük harfe duyarsız başlık öneki filtresi.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeniden eskiye), `commentCount` (en çok yorumlu önce) veya `title` (alfabetik).
    'has_comments' => True, // bool | true ise, yalnızca en az bir yorumu olan sayfaları döndürür.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]