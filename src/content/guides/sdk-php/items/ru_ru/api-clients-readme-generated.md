---
The SDK предоставляет три класса клиентских API:

- **`DefaultApi`** — методы, аутентифицируемые с помощью API-ключа, для использования на сервере. Настройте API-ключ как показано в [Начало работы](#getting-started-readme-generated).
- **`PublicApi`** — публичные методы, которые не требуют API-ключа; безопасно вызывать из браузеров и мобильных приложений.
- **`ModerationApi`** — методы для панели модерации: просмотр списка, подсчёт, поиск, логирование и экспорт комментариев; действия модерации (удаление/восстановление, пометка, установка статуса на проверку/спам/одобрение, голоса, повторное открытие/закрытие темы); баны (забанить от комментирования, отмена, предварительные сводки перед баном, статус и настройки бана, количество забаненных пользователей); и бейджи & доверие (присвоение/удаление бейджа, ручные бейджи, получение/установка коэффициента доверия, внутренний профиль пользователя). Каждый метод `ModerationApi` принимает параметр `$sso` для аутентификации действующего модератора через SSO.

### Использование PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Публичные методы не требуют API-ключа.
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

### Использование ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO-пейлоад, аутентифицирующий модератора

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```
---