## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | path | Sì |  |
| isFlagged | boolean | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Esempio

[inline-code-attrs-start title = 'Esempio flagCommentPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se desideri usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato come predefinito.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$is_flagged = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->flagCommentPublic($tenant_id, $comment_id, $is_flagged, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->flagCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]