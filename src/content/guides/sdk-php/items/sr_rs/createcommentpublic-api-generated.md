## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| broadcastId | string | query | Да |  |
| sessionId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateCommentPublic200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример createCommentPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$comment_data = new \FastComments\Client\Model\CommentData(); // \FastComments\Client\Model\CommentData
$session_id = 'session_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->createCommentPublic($tenant_id, $url_id, $broadcast_id, $comment_data, $session_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---