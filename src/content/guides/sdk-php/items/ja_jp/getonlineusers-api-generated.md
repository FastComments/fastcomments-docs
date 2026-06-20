---
ページに現在オンラインの閲覧者: 現在そのページに websocket セッションがサブスクライブされている人々を指します。
anonCount + totalCount を返します（ルーム全体の購読者数、列挙しない匿名閲覧者を含む）。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページ URL の識別子（サーバー側でクリーンされます）。 |
| afterName | string | query | いいえ | カーソル: 前のレスポンスの nextAfterName を渡してください。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡してください。afterName が設定されている場合、名前の同一によるエントリの脱落を防ぐために必須です。 |

## レスポンス

戻り値: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## 例

[inline-code-attrs-start title = 'getOnlineUsers の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタムの HTTP クライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | ページ URL の識別子（サーバー側でクリーンされます）。
$after_name = 'after_name_example'; // string | カーソル: 前のレスポンスの nextAfterName を渡してください。
$after_user_id = 'after_user_id_example'; // string | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡してください。afterName が設定されている場合、名前の同一によるエントリの脱落を防ぐために必須です。

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---