管理者が FastComments を LTI 1.3 Advantage ツールとして登録し、機関ポリシーを承認すると、教員は標準の Blackboard 配置ポイントを通じてコースに追加できます。Ultra Course View と Original Course View で手順が異なるため、両方を以下に示します。

#### Ultra Course View

Ultra Course View は 2026 年時点の Blackboard Learn SaaS のデフォルトです。

1. コースを開き、**Course Content** ページに移動します。
2. アウトライン内でコメントスレッドを配置したい場所にホバーまたはタップし、紫の **+**（Add content）ボタンをクリックします。
3. **Content Market** を選択します。Content Market パネルには、機関で承認されたすべての LTI ツールと Building Block 配置が一覧表示されます。
4. **FastComments** タイルを見つけてクリックします。Blackboard は **+** メニューを開いた位置にコンテンツアイテムを作成します。
5. アイテムはデフォルトでアウトラインに「学生に表示」として配置されます（個人のデフォルトで **Hide from students** がオフになっている講師の場合）。デフォルトが **Hidden** の場合、アイテムは非表示で作成され、準備ができたらアイテム行の表示切替で表示に変更します。
6. アイテム名を変更するには、アウトラインのタイトルをクリックして新しいラベルを入力します。学生がアウトラインで見るタイトルは FastComments のスレッド識別子とは独立しているため、いつでも安全に名前を変更できます。

**Content Market** がオプションに表示されない場合、機関側でその配置が非表示になっています。同じ **+** メニューの **LTI Tools** グループにある **More tools** からも同じピッカーにアクセスできます。

#### Original Course View

Original Course View は Learn SaaS で引き続きサポートされており、セルフホストの Learn 9.1（Q4 2024 CU リリースライン）では主要な体験として残っています。

1. コースを開き、**Content Area**（例：コースメニューのデフォルト **Information** または **Content** エリア）に入ります。
2. 右上のトグルで **Edit Mode** をオンにします。
3. アクションバーで **Build Content** をクリックします。
4. **Learning Tools** サブメニューの下で **FastComments** をクリックします。Learning Tools サブメニューは、管理者がツールを登録した後の LTI 1.3 ツール配置から構成されます。表示されない場合は下の「注意点」セクションを参照してください。
5. **Create FastComments** フォームで次を設定します:
   - **Name**: 学生がコンテンツエリアで見るラベル。
   - **Description**: 埋め込まれたスレッドの上に表示される任意のテキスト。
   - **Permit Users to View this Content**: 可用性の Yes/No トグル。
   - **Track Number of Views**: Blackboard のアイテムごとの閲覧統計を取得したい場合に有効化。FastComments は独自に解析を実行します。
   - **Date and Time Restrictions**: 任意の **Display After** / **Display Until** ウィンドウ。
6. 送信します。ツールがコンテンツエリアにクリック可能なアイテムとして表示されます。

#### アイテムやドキュメント内への埋め込み

両方のコースビューで、講師はコンテンツエディタの LTI Advantage ボタンを使って、Item、Document、または任意のリッチテキストフィールド内に FastComments をインラインで埋め込みます。

Ultra Course View:

1. **Document** を作成または編集します。
2. スレッドを表示したいドキュメント本文内で **Add content** をクリックします。
3. エディタのツールバーで **Insert content** メニューを開き、**Content Market**（LTI Advantage / Deep Linking のエントリーポイント）をクリックします。
4. **FastComments** を選択します。FastComments は deep-link ペイロードを返し、Blackboard はドキュメント本文のカーソル位置に埋め込みブロックを挿入します。
5. ドキュメントを保存します。学生はスクロールしてその位置を通過すると、スレッドがインラインで表示されます。

Original Course View:

1. リッチテキスト本文を持つ任意のアイテムを編集します。
2. コンテンツエディタのツールバーで **Add Content** のプラスアイコンをクリックし、**Content Market**（古い Q4 2024 CU では **Add Content from External Tool** と表示）を選択します。
3. **FastComments** を選択します。エディタは deep-linked リソースを参照するプレースホルダーブロックを挿入します。
4. アイテムを送信します。

各 deep-link 埋め込みはそれぞれ独立した FastComments スレッドを生成するため、1 つのアイテムに FastComments ブロックが 2 つ埋め込まれていれば、コメントストリームは 2 つになります。

#### 表示、公開条件、およびグループ制限

FastComments のコンテンツアイテムは、上に重ねられたアクセス制御ルールに関して他の Blackboard コンテンツアイテムと同様に動作します。

- Ultra: 行の表示切替（**Visible to students**, **Hidden from students**, **Conditional availability**）をクリックします。Conditional availability は日付/時間ウィンドウ、成績表アイテムに対するパフォーマンスルール、およびコースグループに対するメンバールールをサポートします。
- Original: アイテムのコンテキストメニューを開き、日付、メンバーシップ、成績、またはレビュー状態でツールを制限するには **Adaptive Release** または **Adaptive Release: Advanced** を選びます。特定のコースグループに制限するにはアイテムで **Set Group Availability** を使用します。

FastComments は Blackboard の判定を尊重します。Blackboard が学生に対してアイテムを非表示にした場合、その学生に対する LTI ランチは発生せず、モデレータビューにも表示されません。

#### 成績表の挙動

FastComments は LTI Advantage の Assignment and Grade Services を通じて成績を報告しません。FastComments のコンテンツアイテムに対して自動的に成績列は作成されません。

もしあなたの Blackboard テナントが、採点メタデータに関係なくすべての新しいコンテンツアイテムに対して自動的に成績列を作成するよう設定されている場合、空の列が表示されることがあります。これを非表示にする方法:

- Ultra: **Gradebook** を開き、列ヘッダーをクリックして **Edit** を選び、**Show to students** と **Include in calculations** をオフにします。あるいは、機関が未採点アイテムの列削除を許可している場合は **Delete** を使用します。
- Original: **Grade Center** を開き、列のシェブロンをクリックして **Hide from Users (on/off)** を選び、必要に応じて **Column Organization** の下で **Hide from Instructor View** を選択します。

#### 学生が見るもの

学生が FastComments アイテムを開くか、埋め込まれたブロックまでスクロールすると:

1. Blackboard は FastComments に LTI 1.3 メッセージを送信してランチします。学生は Blackboard の識別情報（名前、メール、アバター、役割）を使って SSO によりサインインし、ログインフォームを見ることはありません。
2. スレッドは iframe 内でレンダリングされます。スレッド化、返信、メンション、リアクションは、FastComments で設定されたコメントウィジェットの設定に基づいて利用可能になります。
3. 彼らのコメントは Blackboard アカウントに紐づけられます。もし学生が後で Blackboard で名前や写真を編集した場合、次回のランチで FastComments のプロフィールが更新されます。

Blackboard から FastComments への役割マッピング:

- **System Administrator** と **Course Builder** は FastComments の **admin** にマップされます。
- **Instructor** と **Teaching Assistant** は FastComments の **moderator** にマップされます。
- **Student**, **Guest**, および **Observer** は FastComments の **commenter** にマップされます。

モデレータはスレッド内のすべてのコメントに対して（ピン、非表示、バン、削除などの）モデレーションコントロールをインラインで表示します。

#### 公開アクセスの制限（推奨）

デフォルトでは、FastComments のコメントデータは公開読み取り可能です。スレッドの URL や API エンドポイントを推測できる誰でも、そのコメントを Blackboard の外部でも閲覧できます。コースのディスカッションでは、登録された学生のみに閲覧を制限するのがほとんどの場合望ましいです。

あなたの <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">ウィジェットカスタマイズページ</a> を開き、**Require SSO To View Comments** を有効にしたルールを作成し、セキュリティレベルを **Secure SSO** に設定してください。これによりスレッドは署名された LTI ランチ経由でのみ読み込めるようになります。

単一ドメインまたはページにルールをスコープする方法を含む完全な手順は、[Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) を参照してください。

#### スレッドのスコーピング

FastComments は各スレッドを **(Blackboard host, course ID, resource link ID)** によってスコープします。同じコース内の 2 つの FastComments アイテムは 2 つのスレッドを生成します。同じアイテムを 2 つのコースシェルにコピーした場合（たとえばコースコピーを通じて）、コピー時に Blackboard が新しい resource link ID を発行するため 2 つのスレッドが生成されます。コースコピー間で共有スレッドを維持するには、コピーを開始する前に FastComments で明示的なスレッド URN を設定した Deep Linking を使用してください。

#### Blackboard 固有の注意点

**Build Content メニュー（Original）や Content Market（Ultra）に FastComments タイルが表示されない。** 管理者はツールを承認したが、関連する配置をブロックする機関ポリシーを残しています。**Administrator Panel** > **Integrations** > **LTI Tool Providers** に移動して FastComments エントリを編集し、**Course Content Tool**（Original）および **Course Content Tool - allow students** / **Deep Linking content tool**（Ultra）配置の両方が有効になっていることを確認してください。保存してコースページをリフレッシュします。

**起動時に「Tool not configured for this context」または「Tool is not deployed」エラーが出る。** 動的登録時に登録されたデプロイメントスコープが、コースが属する機関コンテキストと一致していません。Blackboard のツールプロバイダエントリで **Deployment ID** がこのテナントの FastComments の LTI 1.3 Configuration ページに表示されているものと一致するか確認してください。もし異なる場合は、配置を削除して新しい登録 URL（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ここで取得</a>）から動的登録を再実行してください。

**Iframe の高さが固定されている、またはコンテンツが切れる。** 一部の Blackboard テナントはデフォルトの LTI iframe-resize postMessage をブロックする厳格な Content Security Policy を導入しています。互換性を最大化するために FastComments は Canvas スタイルの `lti.frameResize` メッセージと IMS 仕様形式の `org.imsglobal.lti.frameResize` メッセージの両方を送信しますが、テナントレベルの CSP 上書きが親リスナーをブロックしている場合があります。管理者に `*.fastcomments.com` が LTI ツールの許可リストに含まれていることと、カスタム CSP ヘッダが postMessage イベントを削っていないことを確認してもらってください。そうすれば追加設定なしにリサイズが動作します。

**コースコピーがスレッドを複製する。** Blackboard のコースコピーは LTI 配置に対して新しい resource link ID を発行するため、コピーされたコースは空のスレッドで開始します。これは想定された動作です。コピーしたコースが元のスレッドを継承する必要がある場合は、コピー前に明示的なスレッド URN を設定した Deep Linking を設定するか、FastComments サポートに連絡してスレッド ID を一括で再マップしてください。

**学生が起動時に一般的な Blackboard エラーを見る。** 原因は欠落または古くなった `email` クレームです。FastComments の機関ポリシーで **User Fields to Send** の下に **Role**, **Name**, **Email Address** が有効になっていることを確認してください。保存後、新しいブラウザセッションで再度起動してください。