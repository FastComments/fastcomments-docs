SDK предоставя три API клиентски класа:

- **`DefaultApi`** — Методите, удостоверени с API ключ, за използване от страна на сървъра. Конфигурирайте API ключ както е показано в [Първи стъпки](#getting-started-readme-generated).
- **`PublicApi`** — публични методи, които не изискват API ключ, безопасни за извикване от браузъри и мобилни приложения.
- **`ModerationApi`** — методи за таблото за модерация: изброяване, броене, търсене, логване и експортиране на коментари; действия по модериране (премахване/възстановяване, маркиране, задаване на статус преглед/спам/одобрение, гласове, повторно отваряне/затваряне на нишка); забрани (забрана от коментиране, отмяна, обобщения преди забрана, статус и предпочитания за забрана, брой забранени потребители); и значки & доверие (награждаване/премахване на значка, ръчни значки, получаване/задаване на фактор на доверие, вътрешен профил на потребителя). Всяки метод на `ModerationApi` приема параметър `$sso` за удостоверяване на действащия модератор чрез SSO.

### Използване на PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Публичните методи не изискват API ключ.
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

### Използване на ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO полезен товар за удостоверяване на модератора

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```