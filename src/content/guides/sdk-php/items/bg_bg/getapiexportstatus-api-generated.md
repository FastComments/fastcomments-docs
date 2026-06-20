## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportStatusResponse.php)

## Пример

[inline-code-attrs-start title = 'getApiExportStatus Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако искате да използвате персонализиран HTTP клиент, подайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е опционално, по подразбиране ще бъде използван `GuzzleHttp\Client`.
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