## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| yearNumber | number | query | No |  |
| monthNumber | number | query | No |  |
| dayNumber | number | query | No |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantDailyUsagesResponse.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getTenantDailyUsages'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar autorización de clave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Descomente lo siguiente para configurar el prefijo (p.ej. Bearer) para la clave API, si es necesario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, `GuzzleHttp\Client` se usará por defecto.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // cadena
$options = [
    'year_number' => 3.4, // flotante
    'month_number' => 3.4, // flotante
    'day_number' => 3.4, // flotante
    'skip' => 3.4, // flotante
];


try {
    $result = $apiInstance->getTenantDailyUsages($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantDailyUsages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]