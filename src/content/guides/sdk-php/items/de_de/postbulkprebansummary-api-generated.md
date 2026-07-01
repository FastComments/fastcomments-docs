## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| includeByUserIdAndEmail | boolean | query | Nein |  |
| includeByIP | boolean | query | Nein |  |
| includeByEmailDomain | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkPreBanSummary.php)

## Beispiel

[inline-code-attrs-start title = 'postBulkPreBanSummary Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$bulk_pre_ban_params = new \FastComments\Client\Model\BulkPreBanParams(); // \FastComments\Client\Model\BulkPreBanParams
$options = [
    'include_by_user_id_and_email' => True, // bool
    'include_by_ip' => True, // bool
    'include_by_email_domain' => True, // bool
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->postBulkPreBanSummary($tenant_id, $bulk_pre_ban_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBulkPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]