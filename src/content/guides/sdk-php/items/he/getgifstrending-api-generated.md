## פרמטרים

| שם | סוג | מיקום | דרוש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| locale | string | query | לא |  |
| rating | string | query | לא |  |
| page | number | query | לא |  |

## תגובה

מחזיר: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsTrendingResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getGifsTrending'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש `GuzzleHttp\ClientInterface`.
    // זה אופציונאלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'locale' => 'locale_example', // string
    'rating' => 'rating_example', // string
    'page' => 3.4, // float
];


try {
    $result = $apiInstance->getGifsTrending($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsTrending: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]