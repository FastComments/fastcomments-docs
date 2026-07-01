## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Odpowiedź

Zwraca: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PostRemoveCommentApiResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład postRemoveComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Jeśli chcesz użyć własnego klienta HTTP, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, `GuzzleHttp\Client` będzie użyty domyślnie.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // ciąg znaków
$comment_id = 'comment_id_example'; // ciąg znaków
$options = [
    'broadcast_id' => 'broadcast_id_example', // ciąg znaków
    'sso' => 'sso_example', // ciąg znaków
];


try {
    $result = $apiInstance->postRemoveComment($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postRemoveComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---