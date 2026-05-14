Once an administrator has registered FastComments as an LTI 1.3 Advantage tool and approved the institution policies, instructors add it to courses through the standard Blackboard placement points. The exact steps differ between Ultra Course View and Original Course View, so both are covered below.

#### Ultra Course View

Ultra Course View is the default in Blackboard Learn SaaS as of 2026.

1. コースを開き、**Course Content** ページに移動します。
2. アウトライン内でコメントスレッドを配置したい位置にカーソルを合わせるかタップし、紫色の **+**（Add content）ボタンをクリックします。
3. **Content Market** を選択します。Content Market パネルには、機関で承認されたすべての LTI ツールと Building Block の配置が一覧表示されます。
4. **FastComments** タイルを見つけてクリックします。Blackboard は、**+** メニューを開いた位置にコンテンツ項目を作成します。
5. 項目はデフォルトでアウトラインに「Visible to students」エントリとして配置されます（個人のデフォルトで **Hide from students** がオフになっている講師の場合）。デフォルトが **Hidden** の場合、項目は非表示で作成され、準備ができたら項目行の可視性セレクタを切り替えて表示させます。
6. 項目名を変更するには、アウトラインのタイトルをクリックして新しいラベルを入力します。学生がアウトラインで見るタイトルは FastComments のスレッド識別子とは独立しているため、タイトルの変更はいつでも安全に行えます。

もし **Content Market** がオプションに表示されない場合、機関がその配置を非表示にしています。同じ **+** メニューの **LTI Tools** グループ内にある **More tools** から同じピッカーにアクセスすることもできます。

#### Original Course View

Original Course View は Learn SaaS で引き続きサポートされており、セルフホストの Learn 9.1 サイト（Q4 2024 CU リリースライン）では主要なエクスペリエンスのままです。

1. コースを開き、**Content Area**（たとえば、コースメニューのデフォルト **Information** または **Content** エリア）に入ります。
2. ページ右上のトグルで **Edit Mode** をオンにします。
3. アクションバーで **Build Content** をクリックします。
4. **Learning Tools** サブメニューの下で **FastComments** をクリックします。Learning Tools サブメニューは、管理者がツールを登録した後の LTI 1.3 ツール配置から生成されます。表示されない場合は下の「gotchas」セクションを参照してください。
5. **Create FastComments** フォームで次を設定します:
   - **Name**: コンテンツエリアで学生が目にするラベル。
   - **Description**: 埋め込みスレッドの上に表示される任意のテキスト。
   - **Permit Users to View this Content**: Yes/No の可用性トグル。
   - **Track Number of Views**: Blackboard のアイテム別閲覧統計が必要な場合に有効にします。FastComments は独自にアナリティクスを実行します。
   - **Date and Time Restrictions**: 任意の **Display After** / **Display Until** ウィンドウ。
6. 送信します。ツールはコンテンツエリア内のクリック可能な項目として表示されます。

#### Embedding Inside an Item or Document

両方のコースビューで、講師は Content Editor の LTI Advantage ボタンを使って Item、Document、または任意のリッチテキストフィールド内に FastComments をインライン埋め込みできます。

Ultra Course View:

1. **Document** を作成または編集します。
2. スレッドを表示したいドキュメント本文内で **Add content** をクリックします。
3. エディタのツールバーで **Insert content** メニューを開き、**Content Market**（LTI Advantage / Deep Linking のエントリポイント）をクリックします。
4. **FastComments** を選びます。FastComments はディープリンクのペイロードを返し、Blackboard はカーソル位置に埋め込みブロックをドキュメント本文に挿入します。
5. ドキュメントを保存します。学生はスクロールしてその位置を通過すると、インラインでレンダリングされたスレッドを確認できます。

Original Course View:

1. リッチテキスト本文を持つ任意の項目を編集します。
2. Content Editor のツールバーで **Add Content** のプラスアイコンをクリックし、**Content Market**（古い Q4 2024 CU では **Add Content from External Tool** と表示）を選択します。
3. **FastComments** を選びます。エディタはディープリンクされたリソースを参照するプレースホルダブロックを挿入します。
4. 項目を送信します。

各ディープリンク埋め込みはそれぞれ独立した FastComments スレッドを生成するため、1つの項目に FastComments ブロックが2つ埋め込まれている場合は、2つの独立したコメントストリームになります。

#### Visibility, Release Conditions, and Group Restrictions

FastComments のコンテンツ項目は、上に重ねられたアクセス制御ルールについて他の Blackboard コンテンツ項目と同様に動作します。

- Ultra: 行の可視性セレクタ（**Visible to students**, **Hidden from students**, **Conditional availability**）をクリックします。Conditional availability は日付/時刻ウィンドウ、成績表アイテムに対するパフォーマンスルール、コースグループに対するメンバールールをサポートします。
- Original: 項目のコンテキストメニューを開き、**Adaptive Release** または **Adaptive Release: Advanced** を選択して、日付、メンバーシップ、成績、またはレビュー状況でツールを制限します。項目に対して **Set Group Availability** を使用して特定のコースグループに制限します。

FastComments は Blackboard のゲートが決定する内容を尊重します。Blackboard が学生に対して項目を非表示にする場合、その学生に対しては LTI 起動が実行されず、モデレータビューに表示されません。

#### Gradebook Behavior

FastComments は LTI Advantage Assignment and Grade Services を通じて成績を返送しません。FastComments のコンテンツ項目用に成績列が自動作成されることはありません。

Blackboard テナントがすべての新しいコンテンツ項目に対して成績メタデータにかかわらず自動的に成績列を作成するよう設定されている場合は、空の列が表示されることがあります。非表示にするには:

- Ultra: **Gradebook** を開き、列ヘッダをクリックして **Edit** を選び、**Show to students** と **Include in calculations** をオフにします。あるいは、機関が未採点項目の列削除を許可している場合は **Delete** を使用します。
- Original: **Grade Center** を開き、列のチェブロンをクリックして **Hide from Users (on/off)** を選び、必要に応じて **Column Organization** の下で **Hide from Instructor View** を選択します。

#### What Students See

学生が FastComments 項目を開くか埋め込みブロックまでスクロールすると:

1. Blackboard は FastComments に LTI 1.3 メッセージを起動します。学生は Blackboard の資格情報（名前、メール、アバター、役割）を使用した SSO によりログインし、ログインフォームを見ることはありません。
2. コメントスレッドが iframe 内にレンダリングされます。スレッド化、返信、メンション、リアクションは FastComments のコメントウィジェット設定に基づいて利用可能です。
3. 学生のコメントはその Blackboard アカウントに紐づけられます。学生が後で Blackboard で名前や写真を編集した場合、次回の起動で FastComments のプロフィールが更新されます。

Blackboard から FastComments へのロールマッピング:

- **System Administrator** and **Course Builder** map to FastComments **admin**.
- **Instructor** and **Teaching Assistant** map to FastComments **moderator**.
- **Student**, **Guest**, and **Observer** map to FastComments **commenter**.

モデレータはスレッド内の各コメントに対してピン、非表示、バン、削除などのモデレーションコントロールをインラインで見られます。

#### Thread Scoping

FastComments は各スレッドを **(Blackboard host, course ID, resource link ID)** によってスコープします。同じコース内の 2 つの FastComments 項目は 2 つのスレッドを生成します。同じ項目を 2 つのコースシェルにコピーした場合（たとえばコースコピー経由）、コピー時に Blackboard が新しい resource link ID を発行するため 2 つのスレッドが生成されます。コースコピー間で共有スレッドを維持するには、コピーを開始する前に FastComments 側で明示的なスレッド URN を設定した Deep Linking を使用してください。

#### Blackboard-Specific Gotchas

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** 管理者はツールを承認したものの、該当する配置をブロックする機関ポリシーを残している可能性があります。**Administrator Panel** > **Integrations** > **LTI Tool Providers** に移動し、FastComments エントリを編集して、**Course Content Tool** (Original) および **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) の配置が有効になっていることを確認します。保存してコースページを更新してください。

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** ダイナミック登録時に登録されたデプロイメントスコープが、コースが属する機関のコンテキストと一致していません。Blackboard のツールプロバイダエントリで、**Deployment ID** がこのテナントの FastComments の LTI 1.3 Configuration ページに表示されているものと一致しているか確認してください。異なる場合は、配置を削除して新しい登録 URL からダイナミック登録を再実行します（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ここで取得できます</a>）。

**Iframe height looks fixed or content gets cut off.** 一部の Blackboard テナントはデフォルトの LTI iframe-resize postMessage をブロックする厳格な Content Security Policy を導入しています。互換性を最大化するために FastComments は Canvas スタイルの `lti.frameResize` メッセージと IMS 仕様形式の `org.imsglobal.lti.frameResize` メッセージの両方を送出しますが、テナントレベルの CSP オーバーライドが親リスナーをブロックすることがあります。管理者に `*.fastcomments.com` が LTI ツールの許可リストに含まれていること、およびカスタム CSP ヘッダが postMessage イベントを削除していないことを確認するよう依頼してください。そうすればリサイズは追加設定なしに機能します。

**Course copy duplicates threads.** Blackboard のコースコピーは LTI 配置に対して新しい resource link ID を発行するため、コピーされたコースは空のスレッドから開始します。これは想定された動作です。コピー先のコースが元のスレッドを継承する必要がある場合は、コピー前に明示的なスレッド URN を設定した Deep Linking を設定するか、FastComments サポートに連絡してスレッド ID を一括でリマップしてください。

**Student sees a generic Blackboard error on launch.** 原因は欠落しているか古くなった `email` クレームです。FastComments の機関ポリシーで **User Fields to Send** の下に **Role**, **Name**, **Email Address** が有効になっていることを確認してください。保存し、新しいブラウザセッションで再度起動してください。