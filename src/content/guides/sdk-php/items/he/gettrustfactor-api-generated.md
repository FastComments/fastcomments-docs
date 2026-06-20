---
## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | לא |  |
| sso | string | query | לא |  |

## תשובה

מחזיר: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserTrustFactorResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTrustFactor'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, ייעשה שימוש ב-`GuzzleHttp\Client` כברירת מחדל.
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getTrustFactor($user_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getTrustFactor: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---