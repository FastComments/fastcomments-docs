The SDK излаже три API клијентске класе:

- **`DefaultApi`** — Метoде аутентификоване API кључем за употребу на страни сервера. Конфигуришите API кључ као што је приказано у [Početak rada](#getting-started-readme-generated).
- **`PublicApi`** — јавне методе које не захтијевају API кључ, безбједно их је позивати из прегледача и мобилних апликација.
- **`ModerationApi`** — методе за надзорну плочу модератора: листање, бројање, претрага, логовање и извоз коментара; модерацијске акције (уклони/врати, означи, постави статус рецензије/спам/одобрења, гласови, поново отвори/затвори нит); забране (забрана коментарисања, поништавање, сажеци прије забране, статус и поставке забране, бројеви забрањених корисника); и значке и повјерење (додијели/уклони значку, ручне значке, дохвати/постави фактор повјерења, интерни профил корисника). Свaka `ModerationApi` метода прихвата `$sso` параметар за аутентификацију активног модератора путем SSO.

### Korišćenje PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Javne metode ne zahtijevaju API ključ.
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

### Korišćenje ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO payload koji autentifikuje moderatora

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```