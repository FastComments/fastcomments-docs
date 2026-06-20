---
## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sí |  |
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`PreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PreBanSummary.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getPreBanSummary'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará por defecto.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // cadena
$include_by_user_id_and_email = True; // booleano
$include_by_ip = True; // booleano
$include_by_email_domain = True; // booleano
$sso = 'sso_example'; // cadena

try {
    $result = $apiInstance->getPreBanSummary($comment_id, $include_by_user_id_and_email, $include_by_ip, $include_by_email_domain, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---