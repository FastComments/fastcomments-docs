SDK излаже три класе клијентских API-ја:

- **`DefaultApi`** - Методи аутентификовани API кључем за серверско коришћење. Конфигуришите API кључ као што је приказано у [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - Јавни методи који не захтевају API кључ, безбедни за позив из прегледача и мобилних апликација.
- **`ModerationApi`** - Обиман скуп живих и брзих API-ја за модерацију. Сваки `ModerationApi` метод прихвата параметар `$sso` и може се аутентификовати преко SSO или FastComments.com сесијског колачића.

### Коришћење PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Јавни методи не захтевају API кључ.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // стринг
$url_id = 'url_id_example'; // стринг

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### Коришћење ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // стринг - SSO податак који аутентификује модератора

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```