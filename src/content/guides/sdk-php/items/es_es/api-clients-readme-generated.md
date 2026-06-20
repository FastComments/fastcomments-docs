El SDK expone tres clases de cliente de API:

- **`DefaultApi`** — métodos autenticados mediante clave de API para uso del lado del servidor. Configure una clave de API como se muestra en [Primeros pasos](#getting-started-readme-generated).
- **`PublicApi`** — métodos públicos que no requieren una clave de API, seguros para llamar desde navegadores y aplicaciones móviles.
- **`ModerationApi`** — métodos para el panel de moderación: listado, conteo, búsqueda, registro y exportación de comentarios; acciones de moderación (eliminar/restaurar, marcar, establecer estado de revisión/spam/aprobación, votos, reabrir/cerrar hilo); baneos (banear de comentar, deshacer, resúmenes de pre-baneo, estado y preferencias de baneo, conteos de usuarios baneados); y distintivos y confianza (otorgar/quitar distintivo, distintivos manuales, obtener/establecer factor de confianza, perfil interno del usuario). Cada método de `ModerationApi` acepta un parámetro `$sso` para autenticar al moderador actuante mediante SSO.

### Uso de PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Los métodos públicos no requieren una clave de API.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### Uso de ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - carga útil SSO que autentica al moderador

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```