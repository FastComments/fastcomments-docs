D2L Brightspace は LTI Advantage 管理インターフェースを通じて Dynamic Registration を公開しています。管理者アクセスが必要です。

#### 登録画面を開く

1. 管理者として Brightspace インスタンスにサインインします。
2. **Admin Tools** > **Manage Extensibility** > **LTI Advantage** に移動します。
3. **Register Tool** をクリックします。（直接の URL は `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create` です。）

#### URL を貼り付ける

登録フォームが表示されます。重要な項目は **Tool initiation registration endpoint**（一部の Brightspace バージョンでは "Tool Initiation Registration URL" と表記されます）です。

FastComments の登録 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ここで取得できます</a>) をそのフィールドに貼り付けます。他のフィールドは空のままにしておいてください — 登録ハンドシェイク中に FastComments が自動的に入力します。

**Register** をクリックします。

#### ツールを承認する

Brightspace は FastComments と通信するポップアップを開き、キーを交換して確認画面を表示します。登録が完了するとポップアップは自動で閉じます。

新しいツールは LTI Advantage のツール一覧に表示されます。デフォルトでは Brightspace は新しいツールを **disabled** としてマークします — コースで使用できるようにトグルを **enabled** に切り替えてください。

#### デプロイメントを追加する

Brightspace では、LTI ツールがコースで使用される前に **deployment** が必要です:

1. 新しく登録した FastComments ツールを開きます。
2. **View Deployments** > **New Deployment** をクリックします。
3. デプロイメントに名前を付け（例: "FastComments - All Courses"）、利用可能にする組織単位を選択して保存します。

このデプロイメントを通じて最初に起動すると、FastComments は構成レコードに `deployment_id` を固定します — 同一クライアントの別のデプロイメントからのその後の起動は、再登録しない限り拒否されます。