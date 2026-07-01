## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| page | integer | query | Не |  |
| limit | integer | query | Не |  |
| skip | integer | query | Не |  |
| asTree | boolean | query | Не |  |
| skipChildren | integer | query | Не |  |
| limitChildren | integer | query | Не |  |
| maxTreeDepth | integer | query | Не |  |
| urlId | string | query | Не |  |
| userId | string | query | Не |  |
| anonUserId | string | query | Не |  |
| contextUserId | string | query | Не |  |
| hashTag | string | query | Не |  |
| parentId | string | query | Не |  |
| direction | string | query | Не |  |
| fromDate | integer | query | Не |  |
| toDate | integer | query | Не |  |

## Response

Връща: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetCommentsResponse.php)

## Example

[inline-code-attrs-start title = 'getComments Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриране на упълномощаване с API ключ: api_key
// Разкоментирайте по-долу, за да зададете представка (например Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'page' => 56, // int
    'limit' => 56, // int
    'skip' => 56, // int
    'as_tree' => True, // bool
    'skip_children' => 56, // int
    'limit_children' => 56, // int
    'max_tree_depth' => 56, // int
    'url_id' => 'url_id_example', // string
    'user_id' => 'user_id_example', // string
    'anon_user_id' => 'anon_user_id_example', // string
    'context_user_id' => 'context_user_id_example', // string
    'hash_tag' => 'hash_tag_example', // string
    'parent_id' => 'parent_id_example', // string
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