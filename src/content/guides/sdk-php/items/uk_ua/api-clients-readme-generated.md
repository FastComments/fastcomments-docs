The SDK надає три класи клієнтів API:

- **`DefaultApi`** — методи, автентифіковані за API-ключем, для використання на сервері. Налаштуйте API-ключ як показано в [Початок роботи](#getting-started-readme-generated).
- **`PublicApi`** — публічні методи, які не потребують API-ключа, безпечні для виклику з браузерів і мобільних додатків.
- **`ModerationApi`** — методи для панелі модератора: перелік, підрахунок, пошук, логування та експорт коментарів; модераційні дії (видалити/відновити, позначити, встановити статус перевірки/спаму/схвалення, голоси, повторне відкриття/закриття треду); заборони (заборона коментування, скасування, попередні підсумки перед баном, статус бану та налаштування, кількість забанених користувачів); та бейджі й довіра (нагородити/зняти бейдж, ручні бейджі, отримати/встановити коефіцієнт довіри, внутрішній профіль користувача). Кожен метод `ModerationApi` приймає параметр `$sso` для автентифікації діючого модератора через SSO.

### Використання PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Публічні методи не потребують API-ключа.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // рядок
$url_id = 'url_id_example'; // рядок

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
$sso = 'sso_example'; // рядок - SSO-пейлоад, що автентифікує модератора

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```