## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nee |  |
| limit | integer | query | Nee |  |
| skip | integer | query | Nee |  |
| asTree | boolean | query | Nee |  |
| skipChildren | integer | query | Nee |  |
| limitChildren | integer | query | Nee |  |
| maxTreeDepth | integer | query | Nee |  |
| urlId | string | query | Nee |  |
| userId | string | query | Nee |  |
| anonUserId | string | query | Nee |  |
| contextUserId | string | query | Nee |  |
| hashTag | string | query | Nee |  |
| parentId | string | query | Nee |  |
| direction | string | query | Nee |  |
| fromDate | integer | query | Nee |  |
| toDate | integer | query | Nee |  |

## Response

Retourneert: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetCommentsResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getComments Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer API-sleutelautorisatie: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Haal de commentaarteken weg om het voorvoegsel (bv. Bearer) voor de API-sleutel in te stellen, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als u een aangepaste http-client wilt gebruiken, geef uw client op die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
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
$from_date = 56; // int
$to_date = 56; // int

try {
    $result = $apiInstance->getComments($tenant_id, $page, $limit, $skip, $as_tree, $skip_children, $limit_children, $max_tree_depth, $url_id, $user_id, $anon_user_id, $context_user_id, $hash_tag, $parent_id, $direction, $from_date, $to_date);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]