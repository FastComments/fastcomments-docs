## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| commentId | string | ścieżka | Tak |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentBanStatusResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentBanStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCommentBanStatus($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCommentBanStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---