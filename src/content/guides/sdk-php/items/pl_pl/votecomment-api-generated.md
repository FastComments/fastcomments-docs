## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | ścieżka | Tak |  |
| commentId | string | ścieżka | Tak |  |
| urlId | string | zapytanie | Tak |  |
| broadcastId | string | zapytanie | Tak |  |
| sessionId | string | zapytanie | Nie |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia voteComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, `GuzzleHttp\Client` będzie użyty jako domyślny.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$url_id = 'url_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$vote_body_params = new \FastComments\Client\Model\VoteBodyParams(); // \FastComments\Client\Model\VoteBodyParams
$session_id = 'session_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->voteComment($tenant_id, $comment_id, $url_id, $broadcast_id, $vote_body_params, $session_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->voteComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---