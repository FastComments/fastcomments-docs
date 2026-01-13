## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| limit | number | query | לא |  |
| skip | number | query | לא |  |
| order | string | query | לא |  |
| after | number | query | לא |  |
| before | number | query | לא |  |

## תגובה

מחזיר: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetAuditLogs200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getAuditLogs'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדרת הרשאת מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// הסירו את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונכם להשתמש בלקוח HTTP מותאם אישית, העבירו את הלקוח שלכם שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$limit = 3.4; // מספר עשרוני
$skip = 3.4; // מספר עשרוני
$order = new \FastComments\Client\Model\\FastComments\Client\Model\SORTDIR(); // \FastComments\Client\Model\SORTDIR
$after = 3.4; // מספר עשרוני
$before = 3.4; // מספר עשרוני

try {
    $result = $apiInstance->getAuditLogs($tenant_id, $limit, $skip, $order, $after, $before);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getAuditLogs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---