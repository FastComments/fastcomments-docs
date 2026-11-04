FastCommentsのサポートが移行を支援することもできますが、ほとんどの移行はサポートスタッフの介入なしに、簡単に実行・監視できます。

以下のプロバイダーのエクスポートデータのインポートをネイティブにサポートしています:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress(プラグイン経由、またはXMLまたはCSVエクスポート経由)
- AnyComment(WordPressのインポート/エクスポート経由)

[こちら](https://fastcomments.com/auth/my-account/manage-data/import)に移動すると、移行するデータを含むファイルをアップロードできます。

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='エクスポートファイルのプロバイダー選択とファイルアップロードフィールドがあるFastCommentsのインポートページ'; title='インポートページのフォーム' app-screenshot-end]

### インポートの監視

FastCommentsは、インポートとエクスポートの処理にジョブ処理システムを使用しています。システムがジョブを検知すると、インポートまたはエクスポートのUIにジョブのステータスが定期的に表示されます。

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='実行中のインポートジョブとジョブ処理システムによって報告されたステータスを表示するインポートページ'; title='インポートジョブのステータス' app-screenshot-end]

インポートとエクスポートのステータスは、アカウントのすべての管理者が閲覧できます。

ジョブが失敗した場合、自動的に再起動されることはありません。インポートを再度実行する必要があります。インポートまたはエクスポートが失敗した場合、システム管理者に自動的に通知されます。問題を特定した場合、支援できるかどうかをお客様にご連絡いたします。

### インポートの再実行

移行によっては、インポートを複数回実行する必要がある場合があります。例えば、テストのために最初のパス移行を行い、その後切り替えの前に最新のデータでインポートを再度実行するのが一般的です。

同じコンテンツを再インポートしても**重複は作成されません**。

### データのセキュリティと有効期限

インポートファイルは外部リクエストから一切アクセスできず、インポートが完了するとすぐにシステムから削除されます。