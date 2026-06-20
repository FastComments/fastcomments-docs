## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|--------------|-------------|
| batchJobId | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportStatusResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getApiExportStatus Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);
$batch_job_id = 'batch_job_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getApiExportStatus($batch_job_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiExportStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]