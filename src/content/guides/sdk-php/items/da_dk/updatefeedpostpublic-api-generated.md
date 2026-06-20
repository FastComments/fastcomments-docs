## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postId | string | path | Ja |  |
| broadcastId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPostResponse.php)

## Eksempel

[inline-code-attrs-start title = 'updateFeedPostPublic Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du ønsker at bruge en brugerdefineret HTTP-klient, skal du give din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // streng
$post_id = 'post_id_example'; // streng
$update_feed_post_params = new \FastComments\Client\Model\UpdateFeedPostParams(); // \FastComments\Client\Model\UpdateFeedPostParams
$broadcast_id = 'broadcast_id_example'; // streng
$sso = 'sso_example'; // streng

try {
    $result = $apiInstance->updateFeedPostPublic($tenant_id, $post_id, $update_feed_post_params, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]