現在ページをオンラインで閲覧しているユーザー: 現在、ウェブソケットセッションがそのページに購読されている人々。  
返却: anonCount + totalCount（部屋全体の購読者数で、匿名ビューアーも含まれますが、列挙は行いません）。

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページ URL 識別子（サーバー側でクリーンアップされたもの）。 |
| afterName | string | query | No | カーソル: 前回のレスポンスから nextAfterName を渡す。 |
| afterUserId | string | query | No | カーソルのタイブレーカー: 前回のレスポンスから nextAfterUserId を渡す。afterName が設定されている場合に必要で、名前が同じエントリが除外されないようにします。 |

## Response

返却: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'getOnlineUsers の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションで、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | ページ URL 識別子（サーバー側でクリーンアップされたもの）。
$options = [
    'after_name' => 'after_name_example', // string | カーソル: 前回のレスポンスから nextAfterName を渡す。
    'after_user_id' => 'after_user_id_example', // string | カーソルのタイブレーカー: 前回のレスポンスから nextAfterUserId を渡す。afterName が設定されている場合に必要で、名前が同じエントリが除外されないようにします。
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]