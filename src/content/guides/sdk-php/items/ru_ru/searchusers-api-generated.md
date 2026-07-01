## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| usernameStartsWith | string | query | No |  |
| mentionGroupIds | array | query | No |  |
| sso | string | query | No |  |
| searchSection | string | query | No |  |

## Ответ

Возвращает: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SearchUsersResult.php)

## Пример

[inline-code-attrs-start title = 'Пример searchUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // строка
$url_id = 'url_id_example'; // строка
$options = [
    'username_starts_with' => 'username_starts_with_example', // строка
    'mention_group_ids' => array('mention_group_ids_example'), // string[]
    'sso' => 'sso_example', // строка
    'search_section' => 'search_section_example', // строка
];


try {
    $result = $apiInstance->searchUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->searchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]