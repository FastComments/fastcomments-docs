Our [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) は、強力な UI ベースのインポート機能を備えています。プラグインをインストールすると、WordPress インストールと FastComments をリンクし、既存のコメントデータをコピーする手順を案内します。

**これは手動で何もコピーしたりダウンロードしたりすることなく行われます。**

移行中は UI にて移行プロセスが表示されます。ほとんどの移行は数分で完了します。

この仕組みは、移行中に WordPress インストールに過度な負荷をかけないよう設計されています。

WordPress からサイトを移行する場合は、プラグインを使用せずに WordPress の XML または CSV エクスポートをインポートできます。詳しくは [Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress) をご覧ください。

### CloudFlare & FireWalls

自動化された WordPress 設定が機能するためには、WordPress インストールに対して呼び出しを行う必要があります。Cloudflare などのファイアウォールがブロックし、統合が失敗することがあります。そのような場合、[ご提供できます](https://fastcomments.com/auth/my-account/help) で統合のためにホワイトリストに追加すべき IP のセットをご提供できます。

### Data Ownership

WordPress 移行の場合、新規または更新されたコメントデータは自動的に裏側で WordPress インストールに同期されます。つまり、コメントは FastComments が直接配信して WordPress の負荷を軽減しますが、**同時に** バックアップとしてデータベースにも保存します。また、FastComments から別のサービスへ切り替えたい場合でも、データはすでに移行済みで最新の状態です。