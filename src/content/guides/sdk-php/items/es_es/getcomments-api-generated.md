## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
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

## Respuesta

Devuelve: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetCommentsResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getComments'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar la autorización de la clave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Descomente abajo para configurar el prefijo (p. ej. Bearer) para la clave API, si es necesario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará por defecto.
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