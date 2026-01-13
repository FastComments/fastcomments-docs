FastCommentsのサポートは移行を支援できますが、ほとんどの移行はサポート担当者の介入
なしで簡単に実行および監視できます。

We natively support importing exports from the following providers:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)

By navigating [こちら](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### インポートの監視

FastCommentsはインポートおよびエクスポートの処理にジョブ処理システムを使用します。システムがジョブを取得すると、
定期的にインポートまたはエクスポートのUIにジョブのステータスを報告します。

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

インポートおよびエクスポートのステータスは、アカウント内のすべての管理者が確認できることに注意してください。

ジョブが失敗した場合、自動的に再起動されません。インポートは再度試行する必要があります。インポートまたはエクスポートのいずれかが失敗した場合、
システム管理者に自動で通知されます。問題が判明した場合は、支援できるかどうか確認するためにご連絡します。

### インポートの再実行

一部の移行では、インポートを複数回実行する必要があります。たとえば、テスト用に最初のパス
を実行してから、切り替える前に最新のデータで再度インポートを実行することが一般的です。

同じコンテンツを再インポートしても**重複は作成されません**。

### データのセキュリティと保存期限

インポートファイルは外部からのリクエストでいかなる方法でもアクセスできず、インポートファイルは当社のシステムから
インポート完了後ただちに削除されます。