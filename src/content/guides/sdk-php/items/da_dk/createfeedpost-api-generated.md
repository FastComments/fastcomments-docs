## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| broadcastId | string | query | Nej |  |
| isLive | boolean | query | Nej |  |
| doSpamCheck | boolean | query | Nej |  |
| skipDupCheck | boolean | query | Nej |  |

## Svar

Returnerer: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPostsResponse.php)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på createFeedPost'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøglegodkendelse: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, send din klient som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_feed_post_params = new \FastComments\Client\Model\CreateFeedPostParams(); // \FastComments\Client\Model\CreateFeedPostParams
$broadcast_id = 'broadcast_id_example'; // string
$is_live = True; // bool
$do_spam_check = True; // bool
$skip_dup_check = True; // bool

try {
    $result = $apiInstance->createFeedPost($tenant_id, $create_feed_post_params, $broadcast_id, $is_live, $do_spam_check, $skip_dup_check);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createFeedPost: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---