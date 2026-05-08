FastComments サポートは移行を手伝うことができますが、ほとんどの場合サポート担当者の介入なしで簡単に実行および監視できます。

次のプロバイダからのエクスポートのインポートをネイティブにサポートしています:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

[こちら](https://fastcomments.com/auth/my-account/manage-data/import) に移動することで、移行するデータを含むファイルをアップロードできます。

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### インポートの監視

FastComments はインポートとエクスポートの処理にジョブ処理システムを使用しています。システムがジョブを受け取ると、インポートまたはエクスポートの UI に定期的にジョブのステータスを報告します。

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

インポートとエクスポートのステータスはアカウント内のすべての管理者が表示できることに注意してください。

ジョブが失敗した場合、自動的に再起動されることはありません。インポートは再度試行する必要があります。インポートまたはエクスポートが失敗した場合、当社のシステム管理者に自動的に通知されます。問題が確認された場合は、支援できるかどうかご連絡します。

### インポートの再実行

一部の移行では、インポートを複数回実行する必要があります。たとえば、テストのために最初の試行的な移行を行い、切り替えの前に最新データで再度インポートを実行することが一般的です。

同じコンテンツを再インポートしても **重複は作成されません**。

### データのセキュリティと保存期間

インポートファイルは外部からのリクエストでアクセスできないようになっており、インポートが完了すると当社のシステムから削除されます。