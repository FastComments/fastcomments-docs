## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nej |  |
| limit | integer | query | Nej |  |
| skip | integer | query | Nej |  |
| asTree | boolean | query | Nej |  |
| skipChildren | integer | query | Nej |  |
| limitChildren | integer | query | Nej |  |
| maxTreeDepth | integer | query | Nej |  |
| urlId | string | query | Nej |  |
| userId | string | query | Nej |  |
| anonUserId | string | query | Nej |  |
| contextUserId | string | query | Nej |  |
| hashTag | string | query | Nej |  |
| parentId | string | query | Nej |  |
| direction | string | query | Nej |  |

## Svar

Returnerer: [`GetComments200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetComments200Response.php)

## Eksempel

[inline-code-attrs-start title = 'getComments Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgleautorisation: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentaren nedenfor for at sætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du give din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$page = 56; // int
$limit = 56; // int
$skip = 56; // int
$as_tree = True; // bool
$skip_children = 56; // int
$limit_children = 56; // int
$max_tree_depth = 56; // int
$url_id = 'url_id_example'; // string
$user_id = 'user_id_example'; // string
$anon_user_id = 'anon_user_id_example'; // string
$context_user_id = 'context_user_id_example'; // string
$hash_tag = 'hash_tag_example'; // string
$parent_id = 'parent_id_example'; // string
$direction = new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(); // \FastComments\Client\Model\SortDirections

try {
    $result = $apiInstance->getComments($tenant_id, $page, $limit, $skip, $as_tree, $skip_children, $limit_children, $max_tree_depth, $url_id, $user_id, $anon_user_id, $context_user_id, $hash_tag, $parent_id, $direction);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]