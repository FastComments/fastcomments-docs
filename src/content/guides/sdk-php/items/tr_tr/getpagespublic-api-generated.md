List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opak sayfalama imleci, önceki bir istekte `nextCursor` olarak döndürülen. Aynı `sortBy` ile ilişkilidir. |
| limit | integer | query | No | 1..200, varsayılan 50 |
| q | string | query | No | İsteğe bağlı, büyük/küçük harfe duyarsız başlık önek filtresi. |
| sortBy | string | query | No | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum önce), ya da `title` (alfabetik). |
| hasComments | boolean | query | No | Doğruysa, yalnızca en az bir yorumu olan sayfaları döndür. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'getPagesPublic Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçirin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Opak sayfalama imleci, önceki bir istekte `nextCursor` olarak döndürülen. Aynı `sortBy` ile ilişkilidir.
    'limit' => 56, // int | 1..200, varsayılan 50
    'q' => 'q_example', // string | İsteğe bağlı, büyük/küçük harfe duyarsız başlık önek filtresi.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum önce), ya da `title` (alfabetik).
    'has_comments' => True, // bool | Doğruysa, yalnızca en az bir yorumu olan sayfaları döndür.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]