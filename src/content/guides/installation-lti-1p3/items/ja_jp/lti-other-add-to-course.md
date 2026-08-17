Once FastComments がプラットフォームに登録されると、インストラクターはプラットフォームの標準的な外部ツールフローを使用してコースコンテンツに追加します。このページは Sakai 23.x と Schoology Enterprise を対象としています。

#### 公開アクセスのロックダウン（推奨）

デフォルトでは、FastComments のコメントデータはどちらのプラットフォームでも公開されて読み取れます。スレッドの URL や API エンドポイントを推測できる人は誰でも、Sakai や Schoology の外部でもコメントを閲覧できます。コースディスカッションでは、閲覧を登録された学生のみに制限することがほぼ必須です。

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">ウィジェットカスタマイズページ</a> を開き、**Require SSO To View Comments** を有効にしたルールを作成し、セキュリティレベルを **Secure SSO** に設定します。これにより、スレッドは署名された LTI 起動を通じてのみロードされます。

完全な手順については、[Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) を参照してください。ルールを単一ドメインまたはページにスコープする方法も含まれます。

#### Sakai

**1. サイトに FastComments を追加**

サイト管理者はサイトごとにツールを有効にします：

1. サイトを開き、左側ナビゲーションの **Site Info** をクリックします。
2. **Manage Tools** をクリックします。
3. **External Tools** リストまでスクロールし、**FastComments** をオンに切り替えます。
4. **Continue** をクリックし、ツールリストを確認したら **Finish** をクリックします。

FastComments がサイトの左側ナビゲーション項目として表示されます。

**2. 左側ナビゲーション項目の順序変更**

**Site Info** > **Tool Order** に移動します。**FastComments** を希望の位置にドラッグし、**Save** をクリックします。この画面からナビラベルの名前を変更したり、学生から非表示にしたりすることもできます。

**3. Lessons ページにインラインで埋め込む**

FastComments をスタンドアロンの左側ナビツールではなく、Lessons ページ内に直接配置するには：

1. サイト内の **Lessons** ツールを開きます。
2. **Add Content** > **Add External Tool** をクリックします。
3. リストから **FastComments** を選択します。
4. FastComments が登録時に Deep Linking を宣伝している場合、Sakai はツールのコンテンツセレクタを開き、スレッドを選択またはラベル付けできます。Deep Linking が宣伝されていない場合、Sakai はデフォルトの起動リンクを挿入します。
5. Lessons アイテムを保存します。

埋め込まれた各インスタンスは、そのリソースリンクにスコープされた独自のスレッドを取得します。

**4. 学生アクセスの権限調整**

Sakai は外部ツールの起動を Realms で制御します。学生が FastComments を起動できることを確認するには：

1. Sakai 管理者としてサインインし、**Administration Workspace** > **Realms** を開きます。
2. 該当する realm を開きます（例: `!site.template.course` または特定のサイト realm）。
3. `access` ロールに `lti.launch` が有効であり、**external.tools** グループのロール権限が付与されていることを確認します。
4. realm を保存します。

サイトレベルのオーバーライドについては、管理者は **Site Info** > **Tool Order** からロールごとに FastComments の表示/非表示を調整できます。

**5. 学生が見るもの**

学生は FastComments の左側ナビ項目をクリック（または埋め込まれた Lessons ブロックまでスクロール）し、直接スレッド化されたコメントビューに移動します。SSO は自動的に行われます：Sakai は LTI 起動時にユーザーの ID を送信し、FastComments はその Sakai アカウントでサインインさせます。

ロールマッピング:

- Sakai `Instructor` → FastComments モデレーター
- Sakai `Admin`（Administration Workspace の管理者）→ FastComments 管理者
- Sakai `Student` / `access` → FastComments コメント投稿者

**6. Sakai の注意点**

- **Tool not visible in Manage Tools.** FastComments が External Tools リストに表示されない場合、Sakai 管理者はツールレジストリ（**Administration Workspace** > **External Tools** > **FastComments**）を開き、**Stealthed** を `false` に設定する必要があります。Stealthed ツールはサイトごとの Manage Tools ピッカーから非表示になります。
- **Launches breaking in shared-session browsers.** Sakai のポータル CSRF トークンはブラウザセッションに紐付いています。学生が異なるタブで 2 つの Sakai サイトにサインインしている、またはセッションが古い場合、起動は 403 を返します。対処法：他の Sakai タブを閉じ、サインアウトしてから再度サインインし、再起動します。管理者はクラスター全体でこの問題が発生した場合、`sakai.csrf.token.cache.ttl` を上げることもできます。
- **Frame embedding.** `sakai.properties` の `lti.frameheight` が十分に大きい（600 以上）ことを確認してください。これにより、Lessons ページ内でコメントスレッドが切り取られません。

#### Schoology

Schoology Enterprise には 2 つのインストールシナリオがあります。ツールをコースに追加する前に、どちらが適用されるか確認してください。

**1. 2 つのインストールシナリオ**

- **(a) Enterprise-level install.** Schoology システム管理者が組織レベルで FastComments をインストールし、すべてのコースまたは特定のコーステンプレートに割り当てました。インストラクターはインストールをスキップし、直接「Add Materials」に進みます。
- **(b) Instructor self-install.** インストラクターは **Course Options** > **External Tools** > **Install LTI Apps** からツールを単一コースにインストールします。自己インストールには、システム管理者が事前に組織レベルで FastComments アプリを承認している必要があります。

**2. FastComments をコース教材として追加**

コース内で：

1. コースを開き、**Materials** に移動します。
2. **Add Materials** > **Add File/Link/External Tool** をクリックします。
3. **External Tool** を選択します。
4. 登録済みツールリストから **FastComments** を選択します。
5. **Name** を設定します（これは学生が教材リストで見る名前です）。任意で **Description** も設定できます。
6. **Enable Grading**（成績パスバック）を **OFF** のままにします。FastComments は Schoology に成績を返さないため、成績パスバックを有効にすると空の成績ブック列が作成されます。
7. **Submit** をクリックします。

教材はコース教材リストに表示され、クリックすると FastComments スレッドが開きます。

**3. リッチテキストエディタによるインライン埋め込み**

システム管理者が登録時に FastComments の Deep Linking 配置を有効にしている場合、インストラクターは任意のリッチテキストフィールド（課題指示、ページ本文、ディスカッションプロンプト）内にコメントスレッドを埋め込むことができます：

1. 対象ページでリッチテキストエディタを開きます。
2. ツールバーの **External Tool**（パズルピース）アイコンをクリックします。
3. **FastComments** を選択します。
4. Deep Linking ダイアログで埋め込みを設定し、**Insert** をクリックします。
5. ページを保存します。

リッチテキストエディタに External Tool ボタンが表示されない場合、このテナントでこのツールの Deep Linking が無効になっています。以下の注意点をご参照ください。

**4. 可視性とセクション割り当て**

Schoology は Course Options を通じてセクションごとにツールの利用可能性をスコープします：

1. コースから **Course Options** > **External Tools** をクリックします。
2. インストールされた各 LTI アプリについて、コース内のすべてのセクションで利用可能にするか、特定のセクションに限定するかを制御します。
3. FastComments を特定のセクションに限定するには、ツールを表示させたくないセクションのチェックを外します。
4. セクションレベルのアクセスは、どのセクションが **Add Materials** > **External Tool** エントリで FastComments を見るかも制御します。

**5. 学生が見るもの**

学生は FastComments の教材（またはインライン埋め込み）をクリックし、スレッド化されたディスカッションに移動します。SSO は Schoology アカウントでの LTI 起動により自動的に行われます。

ロールマッピング:

- Schoology `Administrator` → FastComments 管理者
- Schoology `Instructor` → FastComments モデレーター
- Schoology `Student` → FastComments コメント投稿者

**6. Schoology の注意点**

- **Enterprise-only.** 個人および無料の Schoology アカウントでは LTI 1.3 ツールをインストールできません。テナントが無料プランの場合、Course Options に **External Tools** オプションがありません。FastComments を使用するには Schoology Enterprise にアップグレードしてください。
- **Deep Linking disabled by tenant default.** 一部の Schoology テナントは組織レベルで Deep Linking 配置を制限しています。この場合、インストラクターは **Add Materials** > **External Tool** フローのみを見、リッチテキストエディタの External Tool ボタンは表示されません。インライン埋め込みを有効にするには、システム管理者が **System Settings** > **Integration** > **LTI 1.3** > **FastComments** に移動し、**Content Item / Deep Linking** 配置を有効にして保存します。
- **Per-section assignment override.** FastComments がエンタープライズレベルで割り当てられているが、インストラクターが **Add Materials** で見えない場合、コースのセクションが組織レベルの割り当てから除外されています。システム管理者にセクションを FastComments アプリの割り当てに追加するよう依頼してください。
- **Material name vs. thread identity.** Schoology で教材名を変更しても、コメントスレッドは移動しません。スレッドは LTI リソースリンク ID でキー付けされるため、名前変更は同じスレッドを保持します。教材を削除して再作成すると、新しい空のスレッドが作成されます。

---