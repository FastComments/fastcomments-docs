Past commenters on the page who are NOT currently online. Sorted by displayName.  
Sayfada daha önce yorum yapmış ancak şu anda çevrim dışı olan yorumcular. displayName'e göre sıralanır.

Use this after exhausting /users/online to render a "Members" section.  
`/users/online`'ı tükettiğinizde bir "Members" bölümü oluşturmak için bunu kullanın.

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
commenterName üzerinde imleç sayfalama: sunucu {tenantId, urlId, commenterName} kısmını afterName'den itibaren $gt ile ilerleterek dolaşır, $skip maliyeti yok.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıttan nextAfterName değerini gönderin. |
| afterUserId | string | query | No | İmleç bağlayıcı: önceki yanıttan nextAfterUserId değerini gönderin. afterName ayarlandığında, isim bağları nedeniyle girişlerin düşmemesi için gereklidir. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir HTTP istemcisi kullanmak isterseniz, `GuzzleHttp\ClientInterface` arayüzünü uygulayan istemcinizi geçirin.
    // Bu isteğe bağlıdır, `GuzzleHttp\Client` varsayılan olarak kullanılacaktır.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
$options = [
    'after_name' => 'after_name_example', // string | İmleç: önceki yanıttan nextAfterName değerini gönderin.
    'after_user_id' => 'after_user_id_example', // string | İmleç bağlayıcı: önceki yanıttan nextAfterUserId değerini gönderin. afterName ayarlandığında, isim bağları nedeniyle girişlerin düşmemesi için gereklidir.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]