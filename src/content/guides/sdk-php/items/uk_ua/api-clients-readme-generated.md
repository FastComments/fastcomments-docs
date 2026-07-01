The SDK надає три класи клієнтів API:

- **`DefaultApi`** - методи, автентифіковані за допомогою API-ключа, для використання на сервері. Налаштуйте API-ключ, як показано у розділі [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - публічні методи, які не вимагають API-ключа, безпечні для виклику з браузерів та мобільних додатків.
- **`ModerationApi`** - широким набором живих та швидких API модерації. Кожен метод `ModerationApi` приймає параметр `$sso` і може автентифікуватися через SSO або cookie сесії FastComments.com.

### Використання PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Public methods do not require an API key.
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

### Використання ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO payload authenticating the moderator

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```