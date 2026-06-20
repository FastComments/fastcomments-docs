## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | sti | Ja |  |
| broadcastId | string | forespørgsel | Nej |  |
| sso | string | forespørgsel | Nej |  |

## Respons

Returnerer: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPostResponse.php)

## Eksempel

[inline-code-attrs-start title = 'createFeedPostPublic Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, videregiv din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$create_feed_post_params = new \FastComments\Client\Model\CreateFeedPostParams(); // \FastComments\Client\Model\CreateFeedPostParams
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->createFeedPostPublic($tenant_id, $create_feed_post_params, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---