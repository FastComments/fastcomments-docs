## Parameters

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | path | Так |  |
| postIds | array | query | Так |  |
| sso | string | query | Ні |  |

## Response

Повертає: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FeedPostsStatsResponse.php)

## Example

[inline-code-attrs-start title = 'getFeedPostsStats Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP‑клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, `GuzzleHttp\Client` буде використано за замовчуванням.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // рядок
$post_ids = array('post_ids_example'); // рядок[]
$sso = 'sso_example'; // рядок


try {
    $result = $apiInstance->getFeedPostsStats($tenant_id, $post_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getFeedPostsStats: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]