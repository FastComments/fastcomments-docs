---
私たちの[WordPress プラグイン](https://wordpress.org/plugins/fastcomments/)は、強力な UI ベースのインポート機能を備えています。プラグインをインストールすると、WordPress インストールを FastComments とリンクし、既存のコメントデータをコピーする手順を案内します。

**これは手動で何もコピーしたりダウンロードしたりすることなく行われます。**

移行プロセスは、移行中に UI を通じて表示されます。ほとんどの移行は数分で完了します。

この仕組みは、移行中に WordPress インストールに過度な負荷をかけないよう設計されています。

WordPress からサイトを移行する場合、プラグインを使用せずに WordPress の XML または CSV エクスポートをインポートできます。詳しくは[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress)をご覧ください。

### CloudFlare とファイアウォール

自動化された WordPress 設定が機能するためには、WordPress インストールに対して呼び出しを行う必要があります。Cloudflare のようなファイアウォールがブロックすると、統合が失敗することがあります。そのような場合、[私たちは IP のリストを提供し](https://fastcomments.com/auth/my-account/help)統合のためにホワイトリストに追加できます。

### データ所有権

私たちの WordPress 移行の場合、新規または更新されたコメントデータは自動的にバックグラウンドで WordPress インストールに同期されます。つまり、コメントは FastComments が直接配信して WordPress の負荷を軽減しますが、**同時に** データベースにバックアップとして保存します。これにより、FastComments から別のサービスに切り替えたい場合でも、データはすでに移行され、最新の状態になっています。

---