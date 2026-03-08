#### Canvas LTI 設定に移動

FastComments アカウントにログインし、<a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">マイアカウント &gt; Canvas LTI 設定</a> に移動します。

#### 新しい LTI 設定を作成する

「**LTI を有効にする**」チェックボックスをオンにします。設定項目が表示されます:

- **Configuration Name** - この設定を識別するためのオプションのラベル（複数の Canvas インスタンスを接続する場合に便利）。
- **Platform URL** - あなたの Canvas インスタンスの URL（例: `https://yourschool.instructure.com`）。現時点では空欄のままにしておき、Developer Key を作成した後で入力できます。
- **Client ID** - 現時点では空欄のままにします。Canvas で Developer Key を作成した後に入力します。
- **Deployment ID** - 現時点では空欄のままにします。
- **Comment Style** - Comments、Collab Chat、または Both の中から選択します。詳細は [コメントスタイル](#canvas-lms-commenting-styles) を参照してください。

設定を作成するには **追加** をクリックします。

#### 設定用 URL をコピーする

保存後、3 つの URL が表示されます:

- **Configuration URL** - Developer Key を作成する際に Canvas に貼り付けます。
- **OIDC Login URL** - Canvas が LTI ログインフローで使用します（Configuration URL 経由で自動的に設定されます）。
- **Launch URL** - 学生が FastComments を開いたときに Canvas が呼び出すエンドポイントです（Configuration URL 経由で自動的に設定されます）。

**Configuration URL** をコピーしてください。次のステップで必要になります。