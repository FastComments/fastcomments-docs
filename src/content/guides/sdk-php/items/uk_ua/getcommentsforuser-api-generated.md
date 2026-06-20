## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Ні |  |
| direction | string | query | Ні |  |
| repliesToUserId | string | query | Ні |  |
| page | number | query | Ні |  |
| includei10n | boolean | query | Ні |  |
| locale | string | query | Ні |  |
| isCrawler | boolean | query | Ні |  |

## Відповідь

Повертає: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsForUserResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsForUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // string
$direction = new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(); // \FastComments\Client\Model\SortDirections
$replies_to_user_id = 'replies_to_user_id_example'; // string
$page = 3.4; // float
$includei10n = True; // bool
$locale = 'locale_example'; // string
$is_crawler = True; // bool

try {
    $result = $apiInstance->getCommentsForUser($user_id, $direction, $replies_to_user_id, $page, $includei10n, $locale, $is_crawler);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]