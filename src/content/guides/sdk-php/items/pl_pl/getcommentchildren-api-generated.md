## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|-------|-----|-------------|----------|------|
| tenantId | string | query | Tak |  |
| commentId | string | path | Tak |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIChildCommentsResponse.php)

## Przykład

[inline-code-attrs-start title = 'getCommentChildren Przykład'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\ModerationApi(
//     // Jeśli chcesz używać własnego klienta HTTP, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
//     // To jest opcjonalne, jako domyślny użyty zostanie `GuzzleHttp\Client`.
//     new GuzzleHttp\Client()
// );

$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string


try {
    $result = $apiInstance->getCommentChildren($tenant_id, $comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCommentChildren: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]