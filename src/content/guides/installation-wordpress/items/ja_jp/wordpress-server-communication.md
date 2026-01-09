プラグインが動作するために、トークンはあなたの WordPress データベースとあなたの FastComments アカウントの両方に保存されます。プラグインが弊社のサーバーへリクエストを行うとき、この
トークンを提供します。

あなたの FastComments アカウントに許可されたすべての統合は[こちら](https://fastcomments.com/auth/my-account/manage-data/integrations)で確認できます。

すべての通信は HTTPS 経由で行われます。

すべての通信はあなたの WordPress サーバーから FastComments.com への *アウトバウンド* であり、WordPress インストールへの同期（*バック*）を含みます。これは、実装されている
あなたの WordPress インストールの [cron](https://developer.wordpress.org/plugins/cron/) 設定からの [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) を介して行われます。