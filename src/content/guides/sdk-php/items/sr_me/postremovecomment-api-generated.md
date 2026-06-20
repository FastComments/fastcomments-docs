## Параметри

| Име | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PostRemoveCommentResponse.php)

## Примјер

[inline-code-attrs-start title = 'Примјер postRemoveComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, подразумевано ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // низ
$sso = 'sso_example'; // низ

try {
    $result = $apiInstance->postRemoveComment($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postRemoveComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]