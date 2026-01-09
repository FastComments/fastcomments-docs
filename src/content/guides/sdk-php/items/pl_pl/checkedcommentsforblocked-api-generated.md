## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| commentIds | string | query | Tak | Lista identyfikatorów komentarzy oddzielona przecinkami. |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CheckedCommentsForBlocked200Response.php)

## Przykład

[inline-code-attrs-start title = 'Przykład checkedCommentsForBlocked'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_ids = 'comment_ids_example'; // string | Lista identyfikatorów komentarzy oddzielona przecinkami.
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->checkedCommentsForBlocked($tenant_id, $comment_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->checkedCommentsForBlocked: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]