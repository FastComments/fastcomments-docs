Our [WordPress プラグイン](https://wordpress.org/plugins/fastcomments/) には、強力な UI ベースのインポート機能があります。プラグインをインストールすると、WordPress インストールと FastComments をリンクし、既存のコメントデータをコピーする手順を案内します。

**これは手動でコピーやダウンロードを行うことなく実行されます。**

移行中は UI を通じて進行状況が表示されます。ほとんどの移行は数分で完了します。

この仕組みは、移行中に WordPress インストールに過度な負荷がかからないよう設計されています。

WordPress からサイトを移行する場合は、プラグインを使用せずに WordPress の XML または CSV エクスポートをインポートできます。詳しくは[コメントを新しいサイトへ移行する](/guide-installation-wordpress.html#wordpress-moving-off-wordpress)をご覧ください。

### CloudFlare とファイアウォール

自動化された WordPress 設定が機能するためには、WordPress インストールに対して呼び出しを行う必要があります。Cloudflare などのファイアウォールがブロックし、統合が失敗することがあります。そのような場合、[ご提供できます]（https://fastcomments.com/auth/my-account/help）統合用にホワイトリストに登録すべき IP のセットをご案内します。

### データ所有権

当社の WordPress 移行の場合、新規または更新されたコメントデータは自動的にバックグラウンドで WordPress インストールに同期されます。これは、コメントが FastComments 自体によって配信され、WordPress の負荷を軽減する一方で、**同時に** データベースにバックアップとして保存されることを意味します。また、FastComments から別のサービスへ切り替えたい場合でも、データはすでに移行され、最新の状態になっています。