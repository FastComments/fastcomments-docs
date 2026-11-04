FastCommentsでは、同じAPIを使用して独自の拡張機能を自社で開発しています。これらの拡張機能のミニファイされていないコードは、以下のエンドポイントで閲覧できます:

#### ダークモード

ダークモード拡張機能は、「ダーク」ページが検出された場合に条件付きで読み込まれます。この拡張機能は、コメントウィジェットにいくつかのCSSを追加するだけです。これにより、必要のない場合にダークモードのCSSを読み込む必要がなくなります。

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### モデレーション

現在のユーザーがモデレーターである場合、モデレーション拡張機能を使用します。

これは、クリックベースのイベントリスナーの追加、APIリクエストの実行、コメントメニューやコメントエリアへの追加を行う良い例です。

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### ライブチャット

ライブチャット拡張機能は(他の設定やスタイリングと組み合わせることで)、コメントウィジェットをライブチャットコンポーネントに変えます。

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js