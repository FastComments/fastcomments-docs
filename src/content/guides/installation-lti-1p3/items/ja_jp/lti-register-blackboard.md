Blackboard Learn SaaS と Ultra は LTI 1.3 の動的登録をサポートしています。

#### ツールプロバイダー画面を開く

1. システム管理者として Blackboard にサインインします。
2. **Administrator Panel** > **Integrations** > **LTI Tool Providers** に移動します。
3. **Register LTI 1.3 / LTI Advantage Tool** をクリックします。

「Register LTI 1.1 Provider」のみが表示される場合は、お使いの Blackboard バージョンはまだ LTI 1.3 をサポートしていません — アップグレードするか Blackboard サポートに連絡してください。

#### URL を貼り付ける

**Client ID** / **Registration URL** フィールド（バージョンによって Blackboard の表示が異なります）に FastComments の登録 URL を貼り付けます。送信します。

Blackboard が FastComments と登録ハンドシェイクを行い、確認画面を表示します。

#### 承認と有効化

Blackboard は新しく登録されたツールをデフォルトで **Approved but excluded** とマークします:

1. ツールプロバイダー一覧で FastComments のエントリを見つけます。
2. メニューを開き **Edit** を選択します。
3. **Tool Status** を **Approved** に設定します。
4. **Institution Policies** の下で送信されるユーザーデータ（名前、メール、役割）を確認します。保存します。

これで、教員がコースにコンテンツを追加する際にこのツールを利用できるようになります。