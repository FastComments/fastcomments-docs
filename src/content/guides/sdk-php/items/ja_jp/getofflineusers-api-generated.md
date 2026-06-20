ページ上で過去にコメントしたが現在オンラインではないユーザー。displayNameでソートされています。
/users/online を使い果たした後に「Members」セクションをレンダリングするためにこれを使用してください。
commenterName に対するカーソルページネーション: サーバーは部分インデックス {tenantId, urlId, commenterName} を afterName 以降から $gt を使って前方へ走査します。$skip のコストは発生しません。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページ URL 識別子（サーバー側で正規化されます）。 |
| afterName | string | query | いいえ | カーソル: 前回のレスポンスの nextAfterName を渡します。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー: 前回のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合に必要です。名前が等しい場合にエントリが欠落するのを防ぎます。 |

## Response

戻り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | ページ URL 識別子（サーバー側で正規化されます）。
$after_name = 'after_name_example'; // string | カーソル: 前回のレスポンスの nextAfterName を渡します。
$after_user_id = 'after_user_id_example'; // string | カーソルのタイブレーカー: 前回のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合に必要です。名前が等しい場合にエントリが欠落するのを防ぎます。

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]