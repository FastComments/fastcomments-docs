## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| postId | string | path | Так |  |
| broadcastId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPostResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад updateFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_id = 'post_id_example'; // string
$update_feed_post_params = new \FastComments\Client\Model\UpdateFeedPostParams(); // \FastComments\Client\Model\UpdateFeedPostParams
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->updateFeedPostPublic($tenant_id, $post_id, $update_feed_post_params, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]