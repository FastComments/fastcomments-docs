## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odziv

Vrne: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPostResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer createFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti prilagodljen HTTP odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, `GuzzleHttp\Client` bo uporabljen kot privzeto.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$create_feed_post_params = new \FastComments\Client\Model\CreateFeedPostParams(); // \FastComments\Client\Model\CreateFeedPostParams
$options = [
    'broadcast_id' => 'broadcast_id_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->createFeedPostPublic($tenant_id, $create_feed_post_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---