カウントを取得するためのエンドポイントがいくつかあります。何が必要か、そしてブラウザ、サーバー、またはAPI SDKから取得したいかによって異なります。

## 公開コメント数

上記のウィジェットを使用するか、それらが使用するAPIを使用して公開コメント数を取得できます。これらのAPIは2019年以来変更されておらず、今後も変更されることはありません。

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

これは以下のような構造を返します：

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

`postfix`プロパティは常に含まれています。

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

これは以下のような構造を返します：

[inline-code-attrs-start title = 'Bulk Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "translations": {
        "t174": "0 Comments",
        "t175": "1 Comment",
        "t176": "[count] Comments"
    },
    "counts": {
        "x": 10
    }
}
[inline-code-end]

`counts`オブジェクトはカウントを持つページにのみ入力されます。`translations`マップはウィジェットで使用されるため常に存在します。

### 公開エンドポイントの動作 / キャッシュ

公開エンドポイントにはトラフィックのスパイクを処理するための60秒のキャッシュメカニズムがあります。内部的にはサーバーのメモリ内のスレッドごとのLRUキャッシュであるため、人々が多くのコメントを残しているときにカウントがわずかに変化する（上昇してから一時的に下降する）のが見られる場合があります。

公開エンドポイントは常に*合計*コメント数を返し、ルートコメント数ではありません。

### サーバーサイドAPI / SDK

サーバーからコメントを取得する方法は、[Pages API](/guide-api.html#page-structure)を呼び出してページオブジェクトを取得することです。これには合計コメント数とルートコメント数が含まれています。APIリクエストを手動で構築することなくこのAPIを呼び出すことができるSDKを提供しており、型付きの戻り値を提供します。
