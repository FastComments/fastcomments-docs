## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## Svar

Returnerer: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetCommentsResponse.php)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getComments'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgle autorisation: api_key
// Fjern kommenteringen nedenfor for at konfigurere prefix (fx Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // streng
$options = [
    'page' => 56, // int
    'limit' => 56, // int
    'skip' => 56, // int
    'as_tree' => True, // bool
    'skip_children' => 56, // int
    'limit_children' => 56, // int
    'max_tree_depth' => 56, // int
    'url_id' => 'url_id_example', // streng
    'user_id' => 'user_id_example', // streng
    'anon_user_id' => 'anon_user_id_example', // streng
    'context_user_id' => 'context_user_id_example', // streng
    'hash_tag' => 'hash_tag_example', // streng
    'parent_id' => 'parent_id_example', // streng
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'from_date' => 56, // int
    'to_date' => 56, // int
];


try {
    $result = $apiInstance->getComments($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]