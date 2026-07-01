The SDK expone tres clases cliente de API:

- **`DefaultApi`** - Métodos autenticados con clave API para uso del lado del servidor. Configure una clave API como se muestra en [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - Métodos públicos que no requieren una clave API, seguros para llamar desde navegadores y aplicaciones móviles.
- **`ModerationApi`** - una completa suite de APIs de moderación en tiempo real y rápidas. Cada método `ModerationApi` acepta un parámetro `$sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.

### Usando PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Los métodos públicos no requieren una clave API.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // cadena
$url_id = 'url_id_example'; // cadena

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### Usando ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // cadena - carga útil SSO que autentica al moderador

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
}
```