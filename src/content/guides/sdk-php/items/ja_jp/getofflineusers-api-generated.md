現在オンラインではない、そのページの過去のコメント投稿者。displayName でソートされます。
/users/online を使い切った後にこの API を使用して「メンバー」セクションをレンダリングします。
commenterName に対するカーソルページネーション: サーバーは部分インデックス {tenantId, urlId, commenterName} を afterName から $gt で前方に走査し、$skip のコストは発生しません。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページ URL 識別子(サーバー側でクリーンアップされます)。 |
| afterName | string | query | いいえ | カーソル: 前のレスポンスの nextAfterName を渡します。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合は必須です。同一名のエントリが欠落しないようにするためです。 |

## レスポンス

戻り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これは省略可能で、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | ページ URL 識別子(サーバー側でクリーンアップされます)。
$options = [
    'after_name' => 'after_name_example', // string | カーソル: 前のレスポンスの nextAfterName を渡します。
    'after_user_id' => 'after_user_id_example', // string | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合は必須です。同一名のエントリが欠落しないようにするためです。
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]