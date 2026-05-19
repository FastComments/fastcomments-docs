Blackboard Learn SaaS と Ultra は LTI 1.3 の動的登録（Dynamic Registration）をサポートしています。

#### ツールプロバイダ画面を開く

1. システム管理者として Blackboard にサインインします。
2. **Administrator Panel** > **Integrations** > **LTI Tool Providers** に移動します。
3. **Register LTI 1.3 / LTI Advantage Tool** をクリックします。

もし「Register LTI 1.1 Provider」のみが表示される場合、あなたの Blackboard バージョンはまだ LTI 1.3 をサポートしていません — アップグレードするか Blackboard サポートにお問い合わせください。

#### URL を貼り付ける

FastComments の登録用 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ここで取得できます</a>) を **Client ID** / **Registration URL** フィールドに貼り付けます（Blackboard の表記はバージョンによって異なります）。送信します。

Blackboard が FastComments と登録のハンドシェイクを行い、確認画面を表示します。

#### 承認と有効化

Blackboard は、新しく登録されたツールをデフォルトで **Approved but excluded** とマークします:

1. ツールプロバイダ一覧で FastComments のエントリを探します。
2. メニューを開き、**Edit** を選択します。
3. **Tool Status** を **Approved** に設定します。
4. **Institution Policies** の下で、どのユーザーデータが送信されるか（name, email, role）を確認します。保存します。

これで、講師がコースにコンテンツを追加する際にツールを利用できるようになります。