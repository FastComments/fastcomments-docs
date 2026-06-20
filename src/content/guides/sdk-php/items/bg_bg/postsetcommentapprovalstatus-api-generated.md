## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| approved | boolean | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetCommentApprovedResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за postSetCommentApprovalStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако искате да използвате потребителски HTTP клиент, подайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е опционално, по подразбиране ще се използва `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // низ
$approved = True; // булев
$sso = 'sso_example'; // низ

try {
    $result = $apiInstance->postSetCommentApprovalStatus($comment_id, $approved, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentApprovalStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]