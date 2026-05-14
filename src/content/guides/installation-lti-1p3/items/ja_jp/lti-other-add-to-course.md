FastCommentsがプラットフォームに登録されると、講師はプラットフォーム標準の外部ツールフローを使ってコースコンテンツに追加します。このページでは Sakai 23.x と Schoology Enterprise を扱います。

#### Sakai

**1. Add FastComments to a site**

サイトの管理者はサイトごとにツールを有効化します：

1. サイトを開き、左のナビゲーションで **Site Info** をクリックします。
2. **Manage Tools** をクリックします。
3. **External Tools** リストまでスクロールし、**FastComments** をオンに切り替えます。
4. **Continue** をクリックし、ツール一覧を確認してから **Finish** をクリックします。

これで FastComments がサイトの左ナビ項目として表示されます。

**2. Reorder the left-nav entry**

**Site Info** > **Tool Order** に移動します。**FastComments** を目的の位置にドラッグして **Save** をクリックします。この画面からナビラベルの名前変更や学生からの非表示設定も行えます。

**3. Embed inline in a Lessons page**

FastComments を単独の左ナビツールとしてではなく Lessons ページ内に直接配置するには：

1. サイトで **Lessons** ツールを開きます。
2. **Add Content** > **Add External Tool** をクリックします。
3. リストから **FastComments** を選択します。
4. FastComments が登録時に Deep Linking を宣伝していれば、Sakai はツールのコンテンツセレクタを開き、スレッドを選択またはラベル付けできます。Deep Linking を宣言していない場合、Sakai はデフォルトの起動リンクを挿入します。
5. Lessons アイテムを保存します。

埋め込まれた各インスタンスはそのリソースリンクにスコープされた独自のスレッドを持ちます。

**4. Permission tweaks for student access**

Sakai は外部ツールの起動を Realms を通して制御します。学生が FastComments を起動できることを確認するには：

1. Sakai 管理者としてサインインし、**Administration Workspace** > **Realms** を開きます。
2. 該当する realm を開きます（例：`!site.template.course` または特定のサイト realm）。
3. `access` ロールに `lti.launch` が有効になっていること、そして **external.tools** グループ内のロール権限が付与されていることを確認します。
4. realm を保存します。

サイトレベルのオーバーライドについては、管理者が **Site Info** > **Tool Order** からロールごとのツール表示/非表示を調整できます。

**5. What students see**

学生は左ナビの FastComments 項目をクリックするか（あるいは埋め込みの Lessons ブロックまでスクロールして）スレッド表示に直接到達します。SSO は自動です：Sakai が LTI ランチでユーザーの識別情報を送信し、FastComments はその Sakai アカウントでサインインさせます。

ロールのマッピング：

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai gotchas**

- **Tool not visible in Manage Tools.** External Tools リストに FastComments が表示されない場合、Sakai 管理者はツールレジストリ（**Administration Workspace** > **External Tools** > **FastComments**）を開き、**Stealthed** を `false` に設定する必要があります。Stealthed なツールはサイトごとの Manage Tools ピッカーからは非表示になります。
- **Launches breaking in shared-session browsers.** Sakai のポータル CSRF トークンはブラウザセッションに束縛されています。学生が別のタブで二つの Sakai サイトにサインインしているか、セッションが古い場合、起動で 403 が返ることがあります。対処法：他の Sakai タブを閉じ、サインアウトして再度サインインしてから再度起動してください。クラスター全体で発生する場合、管理者は `sakai.csrf.token.cache.ttl` を引き上げることもできます。
- **Frame embedding.** コメントスレッドが Lessons ページ内で切れてしまわないように、`sakai.properties` の `lti.frameheight` が十分に大きい（600 以上）ことを確認してください。

#### Schoology

Schoology Enterprise にはインストールシナリオが二通りあります。ツールをコースに追加する前にどちらが該当するかを確認してください。

**1. Two installation scenarios**

- **(a) Enterprise-level install.** Schoology システム管理者が組織レベルで FastComments をインストールし、すべてのコースまたは特定のコーステンプレートに割り当てています。講師はインストールをスキップし、直接「Add Materials」に進みます。
- **(b) Instructor self-install.** 講師が単一コースにツールをインストールします（**Course Options** > **External Tools** > **Install LTI Apps**）。セルフインストールには、事前にシステム管理者が組織レベルで FastComments アプリを承認している必要があります。

**2. Add FastComments as a course material**

コース内で：

1. コースを開き **Materials** に移動します。
2. **Add Materials** > **Add File/Link/External Tool** をクリックします。
3. **External Tool** を選択します。
4. 登録済みツールのリストから **FastComments** を選択します。
5. **Name** を設定します（これは学生がマテリアル一覧で見る名前です）。任意で **Description** を入力します。
6. **Enable Grading**（成績パスバック）は **OFF** のままにしてください。FastComments は Schoology に成績を送信しないため、成績パスバックを有効にすると空の成績表列が作成されます。
7. **Submit** をクリックします。

マテリアルがコースのマテリアル一覧に表示され、クリックすると FastComments のスレッドが開きます。

**3. Inline embedding via the Rich Text editor**

システム管理者が登録時に FastComments の Deep Linking 配置を有効にしていれば、講師は任意のリッチテキストフィールド（課題の指示、ページ本文、ディスカッションの説明など）内にコメントスレッドを埋め込めます：

1. 対象ページでリッチテキストエディタを開きます。
2. ツールバーの **External Tool**（パズルピース）アイコンをクリックします。
3. **FastComments** を選択します。
4. ディープリンクダイアログで埋め込みを設定し **Insert** をクリックします。
5. ページを保存します。

リッチテキストエディタに External Tool ボタンが表示されない場合、そのテナントで Deep Linking が無効になっています。下の注意点を参照してください。

**4. Visibility and section assignments**

Schoology は Course Options を通じてセクションごとのツール可用性をスコープします：

1. コースから **Course Options** > **External Tools** をクリックします。
2. インストールされている各 LTI アプリについて、コース内の全セクションで利用可能にするか特定のセクションにのみ割り当てるかを制御できます。
3. FastComments を特定のセクションに制限するには、ツールを見せたくないセクションのチェックを外します。
4. セクションレベルのアクセスは **Add Materials** > **External Tool** の FastComments エントリをどのセクションが見られるかも制御します。

**5. What students see**

学生は FastComments のマテリアルをクリックするか（あるいはインライン埋め込みまでスクロールして）スレッドに到達します。SSO は Schoology の LTI ランチを通じて自動的に行われ、学生は自分の Schoology アカウントでサインインします。

ロールのマッピング：

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology gotchas**

- **Enterprise-only.** 個人用および無料の Schoology アカウントでは LTI 1.3 ツールをインストールできません。テナントが無料プランの場合、**Course Options** に **External Tools** オプションが表示されません。FastComments を使用するには Schoology Enterprise にアップグレードしてください。
- **Deep Linking disabled by tenant default.** 一部の Schoology テナントは組織レベルで Deep Linking 配置を制限しています。その場合、講師は **Add Materials** > **External Tool** のフローのみを見て、リッチテキストエディタ内の External Tool ボタンは表示されません。インライン埋め込みを有効にするには、システム管理者が **System Settings** > **Integration** > **LTI 1.3** > **FastComments** に移動し、**Content Item / Deep Linking** 配置を有効にしてから保存します。
- **Per-section assignment override.** FastComments が組織レベルで割り当てられているが講師が **Add Materials** に表示できない場合、コースのセクションが組織レベルの割り当てから除外されています。システム管理者にそのセクションを FastComments アプリの割り当てに追加するよう依頼してください。
- **Material name vs. thread identity.** Schoology でマテリアルの名前を変更してもコメントスレッドは移動しません。スレッドは LTI リソースリンク ID をキーとしているため、名前を変更しても同じスレッドが保持されます。マテリアルを削除して再作成すると新しい空のスレッドが作成されます。