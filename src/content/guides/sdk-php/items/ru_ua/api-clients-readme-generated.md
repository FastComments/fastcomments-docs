SDK предоставляет три клиентских класса API:

- **`DefaultApi`** — методы, аутентифицированные с помощью API-ключа, для использования на стороне сервера. Настройте API-ключ, как показано в [Начало работы](#getting-started-readme-generated).
- **`PublicApi`** — публичные методы, которые не требуют API-ключа, безопасно вызывать из браузеров и мобильных приложений.
- **`ModerationApi`** — методы для панели модерации: перечисление, подсчёт, поиск, логирование и экспорт комментариев; действия модерации (удаление/восстановление, пометка, установка статуса на проверку/спам/одобрение, голоса, повторное открытие/закрытие темы); баны (запрет комментирования, отмена, предварительные сводки по банам, статус бана и настройки, количество забаненных пользователей); и значки & доверие (назначение/удаление значка, ручные значки, получение/установка коэффициента доверия, внутренний профиль пользователя). Каждый метод `ModerationApi` принимает параметр `$sso` для аутентификации действующего модератора через SSO.

### Использование PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Публичные методы не требуют API-ключа.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // строка
$url_id = 'url_id_example'; // строка

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
$sso = 'sso_example'; // строка - полезная нагрузка SSO, аутентифицирующая модератора

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```