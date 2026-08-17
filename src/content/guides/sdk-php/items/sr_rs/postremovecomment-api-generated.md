## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PostRemoveCommentApiResponse.php)

## Пример

[inline-code-attrs-start title = 'postRemoveComment пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће се користити као подразумевано.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // стринг
$comment_id = 'comment_id_example'; // стринг
$options = [
    'broadcast_id' => 'broadcast_id_example', // стринг
    'sso' => 'sso_example', // стринг
];


try {
    $result = $apiInstance->postRemoveComment($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postRemoveComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]