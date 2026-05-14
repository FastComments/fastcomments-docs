このガイドは、サイト管理者がツールを登録しアクティビティ選択で表示されるように設定した後に、Moodle 4.x コースに FastComments を追加する方法を説明します。FastComments がまだ登録されていない場合は、まず Moodle の登録ガイドを参照してください。

#### コースを編集モードで開く

1. コースの Editing Teacher（またはそれ以上）の権限で Moodle にサインインします。
2. コースを開きます。
3. コースヘッダー右上のスイッチで **Edit mode** をオンにします。

Moodle 4.x では、3.x が使用していた従来の「Add an activity or resource」ドロップダウンが全画面のアクティビティ選択ダイアログに置き換えられました。Moodle 4.5 では同じ選択ダイアログを維持しつつ、上部にスター/お気に入り行が追加されているため、一度 FastComments をピン留めすると後でセクション内から素早くアクセスできます。

#### FastComments アクティビティを追加する

1. 議論を配置したいコースのセクション（トピックまたは週）までスクロールします。
2. そのセクションの下部にある **Add an activity or resource** をクリックします。
3. 選択ダイアログで **FastComments** を選択します。見当たらない場合は下の注意点セクションを参照してください。

アクティビティ設定フォームが開きます。重要なフィールドは次のとおりです：

- **Activity name**（必須）。コースページと成績表に表示されます。例： `Week 3 Discussion`。
- **Activity description**。コメントスレッドの上に表示される任意の導入テキスト。
- **Show description on course page**。アクティビティをクリックしなくても説明を表示したい場合にチェックします。
- **Preconfigured tool**。`FastComments` に設定します（選択ダイアログから起動すると自動選択されます）。変更しないでください。
- **Launch container**。**New window** に設定します。なぜ「Same window」が一部の Moodle 展開で問題を起こすかは注意点セクションを参照してください。
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**。空白のままにします。これらはサイトレベルで Dynamic Registration によって処理されます。

画面下部までスクロールして **Save and return to course**（すぐにアクティビティを開く場合は **Save and display**）をクリックします。

アクティビティはセクション内に FastComments アイコン付きの行として表示されます。学生はその行をクリックしてコメントスレッドを開きます。

#### エディタ内に FastComments をインライン埋め込む

Page、Book の章、Lesson、または Atto や TinyMCE エディタを使用するその他のリソース内でのスレッド設置方法：

1. リソースを編集モードで開きます。
2. スレッドを表示させたい位置にカーソルを置きます。
3. エディタのツールバーで **LTI** / **External tool** ボタンをクリックします。Atto では「Insert LTI Advantage content」とラベルされています。TinyMCE（Moodle 4.3+ のデフォルト）では **More** メニュー内の **External tools** にあります。
4. ツール一覧から **FastComments** を選びます。
5. FastComments がディープリンク選択ダイアログを開きます。スレッドタイトルを確認して **Embed** をクリックします。
6. エディタは LTI プレースホルダブロックを挿入します。リソースを保存します。

埋め込まれた各インスタンスはディープリンクのコンテンツアイテム ID に基づく個別のスレッドであり、1 つの Page に FastComments を3つ埋め込むと 3 つの独立したスレッドになります。

#### アクセス制限とグループ設定

FastComments アクティビティには通常の Moodle アクティビティ設定が適用されます：

- **Common module settings** > **Group mode**。これを **Separate groups** または **Visible groups** に設定しても、FastComments が自動的にグループごとのスレッドに分割されるわけではありません。Moodle のグループモードは成績表やメンバーリストをフィルタするだけです。グループごとに別スレッドを運用するには、グループごとに FastComments アクティビティを追加し、**Restrict access** でスコープを設定してください。
- **Restrict access** > **Add restriction**。標準の Moodle 条件（**Date**, **Grade**, **Group**, **Grouping**, **User profile**、および入れ子の制限セット）をサポートします。**Group** を使って FastComments アクティビティを特定のグループに限定できます。
- **Activity completion**。完了トラッキングを行いたい場合は **Students must view this activity to complete it** に設定します。FastComments は現在、起動以外の完了イベントを Moodle に報告しません。

#### ロールマッピング

FastComments は Moodle が起動時に送信する LTI の `roles` クレームを読み取り、次のようにマッピングします：

- Moodle **Manager** または **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** または **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> 読み取り専用

管理者は任意のコメントを削除したりユーザーを禁止したりスレッド設定を編集できます。モデレーターは起動したスレッド内でコメントの削除や承認ができます。カスタム Moodle ロールは、クローン元のアーキタイプのマッピングを継承します。

#### 学生が見るもの

学生は FastComments アクティビティをクリックするか（または Page や Book 内の埋め込みブロックまでスクロールします）。Moodle は起動（LTI launch）を介して学生の識別情報を FastComments に送信します：

- ログイン画面は表示されません。FastComments は Moodle アカウントで自動的にサインインさせます。
- 表示名、メール、アバターは Moodle から取得されます。
- スレッドは `(Moodle site, course, resource link ID)` にスコープされるため、同じアクティビティを別のコースに複製すると新しいスレッドになります。
- スレッド形式の返信、投票、通知はスタンドアロンの FastComments スレッドと同様に動作します。

#### Moodle の注意点

**FastComments がアクティビティ選択に表示されない。** サイト管理者はツールを登録したが、**Tool configuration usage** を **Show in activity chooser and as a preconfigured tool** に設定していません。これは **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > FastComments タイルの歯車アイコンで修正できます。

**「Same window」に設定すると起動に失敗するか空白のフレームが表示される。** Moodle のセッションクッキーはデフォルトで `SameSite=Lax` を使用しており、一部のブラウザは LTI 1.3 が FastComments から戻る際に使用するクロスサイト POST に対してこれらを削除します。アクティビティで **Launch container** を **New window** に設定してください。エディタに埋め込まれた FastComments の場合は、エディタの埋め込み起動パスが常に新しいウィンドウを開くため、これは必須要件です。

**`iss` クレームはテナント ID ではなく Moodle サイトの URL である。** FastComments は LTI の発行者（issuer）として Moodle サイトの URL（`wwwroot` 設定値）を使用します。Moodle インスタンスを別のドメインに移動するか `wwwroot` を変更すると、既存の FastComments スレッドは古い発行者に紐づいたままになり、新しい起動では一致しなくなります。必要に応じて新しい URL に対してツールを再登録し、FastComments 管理画面でスレッドを移行してください。

**アクティビティのバックアップと復元。** コースをバックアップして別のコースに復元すると、新しい resource link ID が作成されるため、復元された FastComments アクティビティは空のスレッドから開始します。元のコースは元のスレッドを保持します。これは意図された動作でありバグではありません。

**Moodle 4.5 の TinyMCE デフォルト。** Moodle 4.5 は新規インストールで TinyMCE をデフォルトエディタとして出荷します。External tool ボタンの位置はメインツールバーではなく **More**（`...`）メニューの下にあります。4.1 からアップグレードした古いサイトは、管理者がデフォルトを切り替えない限り Atto のままです。

---