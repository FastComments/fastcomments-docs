## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | query | Tak |  |
| commentIds | string | query | Tak | Lista identyfikatorów komentarzy oddzielona przecinkami. |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CheckBlockedCommentsResponse.php)

## Example

[inline-code-attrs-start title = 'Przykład checkedCommentsForBlocked'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć własnego klienta http, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, `GuzzleHttp\Client` będzie użyty jako domyślny.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$comment_ids = 'comment_ids_example'; // string | Lista identyfikatorów komentarzy oddzielona przecinkami.
$sso = 'sso_example'; // string


try {
    $result = $apiInstance->checkedCommentsForBlocked($tenant_id, $comment_ids, $ssoa);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->checkedCommentsForBlocked: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---