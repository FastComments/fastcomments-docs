## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | path | Так |  |
| isFlagged | boolean | query | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Приклад

[inline-code-attrs-start title = 'Приклад flagCommentPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$is_flagged = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->flagCommentPublic($tenant_id, $comment_id, $is_flagged, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->flagCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]