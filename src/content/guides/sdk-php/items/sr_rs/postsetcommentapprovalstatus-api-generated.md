## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| approved | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetCommentApprovedResponse.php)

## Пример

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће бити коришћен као подразумевано.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$approved = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postSetCommentApprovalStatus($comment_id, $approved, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentApprovalStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]