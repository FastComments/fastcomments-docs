## Parametri

| Nome | Tipo | Location | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| batchJobId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportStatusResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio getApiExportStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se vuoi utilizzare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, verrà utilizzato `GuzzleHttp\Client` come predefinito.
    new GuzzleHttp\Client()
);
$batch_job_id = 'batch_job_id_example'; // stringa
$sso = 'sso_example'; // stringa

try {
    $result = $apiInstance->getApiExportStatus($batch_job_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiExportStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]