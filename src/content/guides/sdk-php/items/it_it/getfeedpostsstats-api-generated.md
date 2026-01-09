## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| postIds | array | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsStats200Response.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di getFeedPostsStats'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` verrà usato come predefinito.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_ids = array('post_ids_example'); // string[]
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getFeedPostsStats($tenant_id, $post_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getFeedPostsStats: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]