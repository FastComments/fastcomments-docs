req  
tenantId  
afterId  

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Yes |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |

## Отговор

Връща: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getFeedPosts'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Конфигуриране на упълномощаване с API ключ: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Разкоментирайте по-долу, за да настроите префикс (напр. Bearer) за API ключа, ако е необходимо
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Ако искате да използвате персонализиран HTTP клиент, предайте клиента, който имплементира `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Това е по избор, `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
// низ
$options = [
    'after_id' => 'after_id_example', // string
    // низ
    'limit' => 56, // int
    // цяло
    'tags' => array('tags_example'), // string[]
    // низ[]
];


try {
    $result = $apiInstance->getFeedPosts($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getFeedPosts: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---