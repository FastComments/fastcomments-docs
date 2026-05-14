---
Moodle 4.0+ は External Tool プラグインを通じて LTI 1.3 Dynamic Registration をサポートしています。

#### ツール管理画面を開く

1. サイト管理者として Moodle にサインインします。
2. **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** に移動します。

#### URL を貼り付ける

**Tool URL** とラベル付けされたカードが表示されます。FastComments の登録用 URL をテキストフィールドに貼り付け、**Add LTI Advantage** をクリックします。

Moodle はツールの識別情報と要求している権限を表示する登録画面を開きます。内容を確認して **Activate**（または Moodle のバージョンによっては **Register**）をクリックします。

登録が完了するとポップアップは閉じ、新しい FastComments ツールが **Tools** リストに **Active** ステータスで表示されます。

#### 利用可能にする

デフォルトでは Moodle は新しいツールを「Course tools」リストに追加しますが、アクティビティ選択画面には表示しません。コース全体で FastComments を表示するには:

1. FastComments タイルの歯車アイコンをクリックします。
2. **Tool configuration usage** の下で **Show in activity chooser and as a preconfigured tool** を選択します。
3. 保存します。

これで講師は **Add an activity or resource** > **FastComments** から任意のコースに FastComments を追加できるようになります。

---