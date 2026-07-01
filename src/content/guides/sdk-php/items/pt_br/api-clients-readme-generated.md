O SDK expõe três classes cliente de API:

- **`DefaultApi`** - Métodos autenticados com chave de API para uso no lado do servidor. Configure uma chave de API como mostrado em [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - Métodos públicos que não requerem uma chave de API, seguros para chamar de navegadores e aplicativos móveis.
- **`ModerationApi`** - Um conjunto extenso de APIs de moderação ao vivo e rápidas. Cada método `ModerationApi` aceita um parâmetro `$sso` e pode autenticar via SSO ou um cookie de sessão do FastComments.com.

### Usando PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Métodos públicos não requerem uma chave de API.
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

### Usando ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - payload SSO autenticando o moderador

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```