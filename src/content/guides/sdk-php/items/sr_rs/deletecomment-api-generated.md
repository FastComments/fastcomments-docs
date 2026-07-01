## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| isLive | boolean | query | No |  |

## Одговор

Vraća: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentResult.php)

## Пример

[inline-code-attrs-start title = 'deleteComment Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key => // Конфигуришите ауторизацију API кључа: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed => // Одкоментаришите испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
// If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`. => // Ако желите да користите прилагођени HTTP клиент, проследите ваш клиент који имплементира `GuzzleHttp\ClientInterface`.
// This is optional, `GuzzleHttp\Client` will be used as default. => // Ово је опционално, `GuzzleHttp\Client` ће се користити као подразумевани.

$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`. => // Ако желите да користите прилагођени HTTP клиент, проследите ваш клиент који имплементира `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default. => // Ово је опционално, `GuzzleHttp\Client` ће се користити као подразумевани.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // стринг
$id = 'id_example'; // стринг
$options = [
    'context_user_id' => 'context_user_id_example', // стринг
    'is_live' => True, // бул
];


try {
    $result = $apiInstance->deleteComment($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]