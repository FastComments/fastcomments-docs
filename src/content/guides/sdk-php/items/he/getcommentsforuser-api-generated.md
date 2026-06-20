## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## תגובה

מחזיר: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsForUserResponse.php)

## דוגמה

[inline-code-attrs-start title = 'getCommentsForUser דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
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

---