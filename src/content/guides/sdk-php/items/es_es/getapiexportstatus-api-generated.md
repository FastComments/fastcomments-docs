## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| batchJobId | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportStatusResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getApiExportStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará por defecto.
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