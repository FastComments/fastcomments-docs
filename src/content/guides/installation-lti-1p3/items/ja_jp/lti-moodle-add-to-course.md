このガイドでは、サイト管理者がツールを登録してアクティビティ選択で表示するように設定した後に、Moodle 4.x コースに FastComments を追加する方法を説明します。FastComments がまだ登録されていない場合は、まず Moodle 登録ガイドを参照してください。

#### コースを編集モードで開く

1. コースの編集教師（またはそれ以上）の権限で Moodle にサインインします。
2. コースを開きます。
3. コースヘッダー右上のスイッチで **編集モード** をオンに切り替えます。

Moodle 4.x は 3.x の「アクティビティまたはリソースを追加」ドロップダウンを廃止し、フルスクリーンのアクティビティ選択ダイアログに置き換えました。Moodle 4.5 は同じ選択ダイアログを保持し、上部にスター付き／お気に入り行を追加しているので、一度 FastComments をピン留めしておくと後で見つけやすくなります。

#### FastComments アクティビティを追加する

1. ディスカッションを配置したいコースのセクション（トピックまたは週）までスクロールします。
2. そのセクションの下部にある **アクティビティまたはリソースを追加** をクリックします。
3. 選択ダイアログで **FastComments** を選択します。表示されない場合は、以下の「トラブルシューティング」セクションを参照してください。

アクティビティ設定フォームが開きます。重要なフィールドは次のとおりです:

- **Activity name**（必須）。コースページと成績簿に表示されます。例: `Week 3 Discussion`。
- **Activity description**。コメントスレッドの上に表示される任意の導入テキスト。
- **Show description on course page**。アクティビティを開かなくても説明を表示したい場合はチェックします。
- **Preconfigured tool**。`FastComments` に設定します（選択ダイアログから起動すると自動選択されます）。変更しないでください。
- **Launch container**。**New window** に設定します。いくつかの Moodle 配備で「Same window」が動作しない理由はトラブルシューティングを参照してください。
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**。空欄のままにします。これらはサイトレベルでの動的登録が処理しています。

下までスクロールして **Save and return to course**（またはすぐにアクティビティを開く場合は **Save and display**）をクリックします。

アクティビティは該当セクションに FastComments アイコン付きの行として表示されます。学生はその行をクリックしてコメントスレッドを開きます。

#### エディタ内に FastComments をインライン埋め込む

Page、Book の章、Lesson、または Atto や TinyMCE エディタを使用するその他のリソース内にスレッドを埋め込む場合:

1. リソースを編集モードで開きます。
2. スレッドを表示したい位置にカーソルを置きます。
3. エディタのツールバーで **LTI** / **External tool** ボタンをクリックします。Atto では「Insert LTI Advantage content」と表示されます。TinyMCE（Moodle 4.3+ のデフォルト）では **More** メニューの中の **External tools** にあります。
4. ツール一覧から **FastComments** を選びます。
5. FastComments がディープリンクのピッカーを開きます。スレッドタイトルを確認して **Embed** をクリックします。
6. エディタは LTI プレースホルダーブロックを挿入します。リソースを保存します。

埋め込まれた各インスタンスはディープリンクのコンテンツアイテム ID をキーとする個別のスレッドなので、1つの Page に FastComments の埋め込みが 3 つある場合は独立した 3 つのスレッドになります。

#### アクセス制限とグループ設定

FastComments アクティビティには標準の Moodle アクティビティ設定が適用されます:

- **Common module settings** > **Group mode**。これを **Separate groups** または **Visible groups** に設定しても、FastComments が自動的にグループごとのスレッドに分割されるわけではありません。Moodle のグループモードは成績簿とメンバーリストをフィルタリングするだけです。グループごとに別々のスレッドを運用するには、各グループごとに FastComments アクティビティを追加し、**Restrict access** で各アクティビティの対象範囲を制限してください。
- **Restrict access** > **Add restriction**。標準の Moodle 条件（**Date**, **Grade**, **Group**, **Grouping**, **User profile**, およびネストされた制限セット）に対応しています。特定のグループに FastComments アクティビティをロックするには **Group** を使用します。
- **Activity completion**。完了トラッキングを有効にしたい場合は **Students must view this activity to complete it** に設定します。現時点で FastComments は起動以外の完了イベントを Moodle に返しません。

#### ロールマッピング

FastComments は Moodle が起動時に送信する LTI の `roles` クレームを読み取り、次のようにマップします:

- Moodle **Manager** または **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** または **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> 読み取り専用

管理者は任意のコメントを削除したり、ユーザーを禁止したり、スレッド設定を編集できます。モデレーターは起動したスレッド内でコメントの削除や承認が可能です。カスタム Moodle ロールは、それがクローンされたアーキタイプのマッピングを継承します。

#### 学生が見るもの

学生は FastComments アクティビティをクリックするか（または Page や Book 内の埋め込みブロックまでスクロールします）。Moodle は起動時に LTI を通じて学生の識別情報を FastComments に送ります:

- ログイン画面は表示されません。FastComments は Moodle アカウントを使って学生をサインインさせます。
- 表示名、メールアドレス、アバターは Moodle から取得されます。
- スレッドは `(Moodle site, course, resource link ID)` にスコープされるため、同じアクティビティを別のコースに複製すると新しいスレッドが作成されます。
- スレッド形式の返信、投票、通知はスタンドアロンの FastComments スレッドと同じように動作します。

#### 公開アクセスを制限する（推奨）

デフォルトでは、FastComments のコメントデータは公開で読み取り可能です。スレッドの URL や API エンドポイントを推測できる人は、Moodle 外部からでもコメントを閲覧できます。コースディスカッションでは、登録済みの学生だけが閲覧できるように制限するのがほとんどの場合望ましいでしょう。

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">ウィジェットカスタマイズページ</a>を開き、**Require SSO To View Comments** を有効にしたルールを作成し、セキュリティレベルを **Secure SSO** に設定して、スレッドが署名された LTI 起動を通じてのみ読み込まれるようにします。

完全な手順（単一サインオンでのスレッド保護方法、ドメインやページ単位でルールをスコープする方法を含む）は、[Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) を参照してください。

#### Moodle のトラブルシューティング

**アクティビティ選択に FastComments が表示されない。** サイト管理者がツールを登録したが、**Tool configuration usage** を **Show in activity chooser and as a preconfigured tool** に設定していません。これを修正するには **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > FastComments タイルのギアアイコンの順に進んでください。

**「Same window」に設定すると起動に失敗するか空白のフレームが表示される。** Moodle のセッションクッキーはデフォルトで `SameSite=Lax` を使用しており、ブラウザによっては LTI 1.3 が FastComments から戻る際に行うクロスサイト POST でこれらのクッキーが削除されます。アクティビティの **Launch container** を **New window** に設定してください。エディタに埋め込まれた FastComments では、エディタ内の起動パスが常に新しいウィンドウをポップアップするため、これはハード要件です。

**`iss` クレームはテナント ID ではなく Moodle サイトの URL です。** FastComments は LTI の発行者として Moodle サイトの URL（`wwwroot` 設定値）を使用します。Moodle インスタンスを新しいドメインに移動するか `wwwroot` を変更すると、既存の FastComments スレッドは古い発行者に紐づいたままになり、新しい起動と一致しなくなります。必要に応じてツールを新しい URL で再登録し、FastComments 管理画面でスレッドを移行してください。

**アクティビティのバックアップと復元。** コースをバックアップして別のコースに復元すると、新しいリソースリンク ID が作成されるため、復元された FastComments アクティビティは空のスレッドで開始されます。元のコースは元のスレッドを保持します。これは意図された動作でありバグではありません。

**Moodle 4.5 の TinyMCE デフォルト。** Moodle 4.5 では新規インストールに対して TinyMCE がデフォルトのエディタとして搭載されています。External tool ボタンの位置はメインツールバーではなく **More**（`...`）メニューの下になります。4.1 からアップグレードした古いサイトは、管理者がデフォルトを切り替えていない限り Atto のままです。