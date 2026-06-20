The SDK пружа три класе клијента API-ја:

- **`DefaultApi`** — методе аутентификоване API кључем за коришћење на серверу. Конфигуришите API кључ као што је приказано у [Почетак рада](#getting-started-readme-generated).
- **`PublicApi`** — јавни методи који не захтевају API кључ, безбедно их је позивати из прегледача и мобилних апликација.
- **`ModerationApi`** — методе за контролну таблу модерације: листање, бројање, претрага, евиденција и извоз коментара; акције модерације (уклони/врати, означи/пријави, подеси статус рецензије/спам/одобравања, гласови, поновно отварање/затварање нити); бановања (бановање од коментарисања, поништавање, прегледи пре-бановања, статус и преференце бана, бројање забрањених корисника); и значке и поверење (додавање/уклањање значке, мануелне значке, добијање/подешавање фактора поверења, интерни кориснички профил). Свака метода `ModerationApi` прихвата параметар `$sso` за аутентификацију модератора који извршава акцију путем SSO.

### Коришћење PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Јавни методи не захтевају API кључ.
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

### Коришћење ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO payload који аутентификује модератора

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```