## Parametry

| Name | Type | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | ścieżka | Tak |  |
| commentId | string | ścieżka | Tak |  |
| broadcastId | string | zapytanie | Tak |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`LockComment200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/LockComment200Response.php)

## Przykład

[inline-code-attrs-start title = 'Przykład unLockComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż swój klient, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie używany będzie `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->unLockComment($tenant_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->unLockComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---