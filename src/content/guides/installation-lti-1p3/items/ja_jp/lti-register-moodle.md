**Moodle を使っていますか？** FastComments は LTI 1.3 よりも緊密に統合された専用の Moodle プラグイン（成績同期フック、より詳細なアクティビティ報告、ネイティブな Moodle 設定 UI）も公開しています。<a href="/guide-installation-moodle.html" target="_blank">Moodle プラグインインストールガイド</a> を参照してください。以下の LTI 1.3 フローは、他の LMS もカバーする単一の登録を行いたい場合、または Moodle 管理者がサードパーティプラグインをインストールしない場合に適した選択です。

Moodle 4.0+ は External Tool プラグインを通じて LTI 1.3 の動的登録をサポートします。

#### ツール管理画面を開く

1. サイト管理者として Moodle にサインインします。
2. **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** に移動します。

#### URL を貼り付ける

**Tool URL** というラベルのカードが表示されます。テキストフィールドに FastComments の登録 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">こちらから取得</a>) を貼り付け、**Add LTI Advantage** をクリックします。

Moodle はツールの識別情報と要求している権限を表示する登録画面を開きます。内容を確認して **Activate**（Moodle のバージョンによっては **Register**）をクリックします。

登録が完了するとポップアップは閉じ、新しい FastComments ツールが **Tools** リストに **Active** ステータスで表示されます。

#### 利用可能にする

デフォルトでは Moodle は新しいツールを「Course tools」リストに追加しますが、アクティビティピッカーには表示しません。コース全体で FastComments を表示するには:

1. FastComments タイルの歯車アイコンをクリックします。
2. **Tool configuration usage** の下で **Show in activity chooser and as a preconfigured tool** を選択します。
3. 保存します。

講師はこれで **Add an activity or resource** > **FastComments** から任意のコースに FastComments を追加できるようになります。