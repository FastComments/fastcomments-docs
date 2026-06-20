## Parameters

| Naam | Type | Location | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Geeft terug: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportStatusResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getApiExportStatus Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Als u een eigen HTTP-client wilt gebruiken, geef dan uw client mee die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` zal als standaard worden gebruikt.
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

---