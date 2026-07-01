## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | път | Да |  |
| commentId | string | път | Да |  |
| broadcastId | string | заявка | Да |  |
| sso | string | заявка | Не |  |

## Отговор

Връща: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ChangeCommentPinStatusResponse.php)

## Пример

[inline-code-attrs-start title = 'pinComment Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонализиран HTTP клиент, предайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // низ
$comment_id = 'comment_id_example'; // низ
$broadcast_id = 'broadcast_id_example'; // низ
$sso = 'sso_example'; // низ


try {
    $result = $apiInstance->pinComment($tenant_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->pinComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]