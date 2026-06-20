## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| includeByUserIdAndEmail | boolean | query | לא |  |
| includeByIP | boolean | query | לא |  |
| includeByEmailDomain | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`PreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PreBanSummary.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPreBanSummary'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, מסור את הלקוח שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי; ברירת המחדל תהיה `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // מחרוזת
$include_by_user_id_and_email = True; // בוליאני
$include_by_ip = True; // בוליאני
$include_by_email_domain = True; // בוליאני
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->getPreBanSummary($comment_id, $include_by_user_id_and_email, $include_by_ip, $include_by_email_domain, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]