## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| batchJobId | string | query | No |  |
| sso | string | query | No |  |

## Odziv

Vrne: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportStatusResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getApiExportStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\ModerationApi(
//     // Če želite uporabiti custom http odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
//     // To je neobvezno, `GuzzleHttp\Client` bo uporabljen kot privzeto.
//     new GuzzleHttp\Client()
// );

$tenant_id = 'tenant_id_example'; // string
$options = [
    'batch_job_id' => 'batch_job_id_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getApiExportStatus($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiExportStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]