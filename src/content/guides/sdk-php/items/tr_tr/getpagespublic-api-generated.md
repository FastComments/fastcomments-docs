---
Bir tenant için sayfaları listeleyin. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır. Her sayfa için çözümlenmiş özel yapılandırmada `enableFChat`'in true olması gerekir. SSO gerektiren sayfalar, talep eden kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| cursor | string | query | Hayır | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalandırma imleci. Aynı `sortBy` ile ilişkili. |
| limit | integer | query | Hayır | 1..200, varsayılan 50 |
| q | string | query | Hayır | İsteğe bağlı, büyük/küçük harfe duyarsız başlık önek filtresi. |
| sortBy | string | query | Hayır | Sıralama düzeni. `updatedAt` (varsayılan, en yeni ilk), `commentCount` (en çok yorum ilk), veya `title` (alfabetik). |
| hasComments | boolean | query | Hayır | Eğer true ise, yalnızca en az bir yoruma sahip sayfaları döndürür. |

## Yanıt

Dönüş: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Örnek

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir http istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalandırma imleci. Aynı `sortBy` ile ilişkili.
$limit = 56; // int | 1..200, varsayılan 50
$q = 'q_example'; // string | İsteğe bağlı, büyük/küçük harfe duyarsız başlık önek filtresi.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni ilk), `commentCount` (en çok yorum ilk), veya `title` (alfabetik).
$has_comments = True; // bool | Eğer true ise, yalnızca en az bir yoruma sahip sayfaları döndürür.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---