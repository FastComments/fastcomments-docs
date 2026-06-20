## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| postId | string | path | Да |  |
| isUndo | boolean | query | Не |  |
| broadcastId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ReactFeedPostResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за reactFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате потребителски HTTP клиент, подайте ваш клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, по подразбиране ще се използва `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_id = 'post_id_example'; // string
$react_body_params = new \FastComments\Client\Model\ReactBodyParams(); // \FastComments\Client\Model\ReactBodyParams
$is_undo = True; // bool
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->reactFeedPostPublic($tenant_id, $post_id, $react_body_params, $is_undo, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->reactFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]