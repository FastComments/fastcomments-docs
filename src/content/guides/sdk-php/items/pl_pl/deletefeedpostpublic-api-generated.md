## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| postId | string | path | Tak |  |
| broadcastId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteFeedPostPublicResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż swój klient, który implementuje `GuzzleHttp\ClientInterface`.
    // To opcjonalne, `GuzzleHttp\Client` zostanie użyty jako domyślny.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_id = 'post_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteFeedPostPublic($tenant_id, $post_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]