FastComments は、コメントをドメイン間で移行する自動化された方法を提供します。

ドメイン移行には、`from` と `to` のドメインが必要です。

これはコメントを**移動**します。コピーは行いません。コメントをコピーしたい場合は、弊社までご連絡ください。

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='from と to のドメインフィールドと、移行されたコメント数を示すドメイン移行ツール'; title='ドメインの移行' app-screenshot-end]

たとえば、FastComments への移行の一部が別のプロバイダーからの移行を含む場合、コメントインポートデータに移行が必要なデータが含まれることがあります。このような場合、インポートを実行し、その後にドメイン移行を行うことができます。

### 進捗の監視

ドメイン移行ツールは、他のデータ管理ツールと同じ FastComments のジョブ処理システムを使用します。

移行が開始されるまでに遅延が生じることがあります。これは、システムが定期的に新しいジョブを処理するかどうかをチェックしているため、正常な動作です。

ジョブが実行されると、移行対象として見つかったコメント数と、これまでに移行されたコメント数が表示されます。

### API を使用したドメインの移行

ドメインは、`type` を `domain` に設定して、[`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post) を使用しても移行できます。進捗は [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id) から取得でき、移行は 90 日間取り消すことが可能です。

---