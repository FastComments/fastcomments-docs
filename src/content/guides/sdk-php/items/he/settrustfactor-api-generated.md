---
## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| userId | string | query | לא |  |
| trustFactor | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetUserTrustFactorResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-setTrustFactor'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי; `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // מחרוזת
$trust_factor = 'trust_factor_example'; // מחרוזת
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->setTrustFactor($user_id, $trust_factor, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->setTrustFactor: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---