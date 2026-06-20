## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | Não |  |
| includeByIP | boolean | query | Não |  |
| includeByEmailDomain | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkPreBanSummary.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de postBulkPreBanSummary'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se você quiser usar um cliente HTTP customizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado por padrão.
    new GuzzleHttp\Client()
);
$bulk_pre_ban_params = new \FastComments\Client\Model\BulkPreBanParams(); // \FastComments\Client\Model\BulkPreBanParams
$include_by_user_id_and_email = True; // bool
$include_by_ip = True; // bool
$include_by_email_domain = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postBulkPreBanSummary($bulk_pre_ban_params, $include_by_user_id_and_email, $include_by_ip, $include_by_email_domain, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBulkPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]