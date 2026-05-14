D2L Brightspace は LTI Advantage 管理インターフェイスを通じて動的登録 (Dynamic Registration) を公開しています。管理者アクセスが必要です。

#### 登録画面を開く

1. 管理者として Brightspace にサインインします。
2. **Admin Tools** > **Manage Extensibility** > **LTI Advantage** に移動します。
3. **Register Tool** をクリックします。 (直接URLは `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create` です。)

#### URL を貼り付ける

登録フォームが表示されます。重要な項目は **Tool initiation registration endpoint**（Brightspace の一部のバージョンでは "Tool Initiation Registration URL" と表記されます）。

そのフィールドに FastComments の登録用 URL を貼り付けます。他の項目は空欄のままにしてください — 登録ハンドシェイク中に FastComments が自動的に入力します。

**Register** をクリックします。

#### ツールを承認する

Brightspace は FastComments と通信してキーを交換し、確認画面を表示するポップアップを開きます。登録が完了するとポップアップは自動的に閉じます。

新しいツールは LTI Advantage のツール一覧に表示されます。既定では Brightspace は新しいツールを **disabled** としてマークします — コースで使用できるようにトグルを **enabled** に切り替えてください。

#### デプロイメントを追加する

Brightspace では、LTI ツールをコースで使用する前に **deployment** が必要です:

1. 新しく登録した FastComments ツールを開きます。
2. **View Deployments** > **New Deployment** をクリックします。
3. デプロイメントに名前を付けます（例: "FastComments - All Courses"）、利用可能にする組織ユニットを選択し、保存します。

このデプロイメントを経由して最初に起動された後、FastComments は構成レコードに `deployment_id` を固定します — 同じクライアントの別のデプロイメントからの以降の起動は、再登録しない限り拒否されます。