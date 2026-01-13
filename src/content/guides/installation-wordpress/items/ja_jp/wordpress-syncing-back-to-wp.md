デフォルトでは、FastComments は毎日あなたの WordPress サイトへ同期します。これは純粋にバックアップ目的であり、データのコピーを保持し続けるため、
それに依存するプラグインのためでもあります。

一部のサイトは大量の読み取りトラフィックに対応できる一方で、データベースの構成が大量の書き込みトラフィックに対応できないことがあるため、保存されたすべてのコメントごとに即座に同期が行われるわけではありません（このため、この作業を FastComments にオフロードしています）。

WordPress への同期スケジュールはプラグインをインストールすることでカスタマイズできます。推奨プラグインは [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description) です。

Steps:

1. WP Crontrol をインストール
2. `Settings -> Cron Schedules` に移動します。
3. `Cron Events` タブに移動します。
4. `fastcomments_cron_hook` を検索します。
5. イベントを編集します。フックを毎時、1日2回、毎日（デフォルト）、または週1回実行するように設定できます。

WordPress への同期は、FastComments プラグインのダッシュボードに移動して `Manually Sync` を選択することで、いつでも手動で実行できます。あなたは
あなたの WP インストールへ同期するか、またはあなたの WP コメントを FastComments サーバーに再アップロードするかを選択できます。