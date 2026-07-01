The SDK udostępnia trzy klasy klientów API:

- **`DefaultApi`** – metody uwierzytelniane kluczem API do użycia po stronie serwera. Skonfiguruj klucz API, jak pokazano w [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** – publiczne metody, które nie wymagają klucza API, bezpieczne do wywoływania z przeglądarek i aplikacji mobilnych.
- **`ModerationApi`** – rozbudowany zestaw szybkich i bieżących API moderacji. Każda metoda `ModerationApi` akceptuje parametr `$sso` i może uwierzytelnić się za pomocą SSO lub pliku cookie sesji FastComments.com.

### Używanie PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Publiczne metody nie wymagają klucza API.
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

### Używanie ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - ładunek SSO uwierzytelniający moderatora

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```