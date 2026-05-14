管理者が FastComments を LTI 1.3 Advantage ツールとして登録し、機関ポリシーを承認すると、講師は標準の Blackboard 配置ポイントを通じてコースに追加します。正確な手順は Ultra Course View と Original Course View で異なるため、両方を以下に説明します。

#### Ultra Course View

Ultra Course View は 2026 年時点で Blackboard Learn SaaS のデフォルトです。

1. コースを開き、**Course Content** ページに移動します。
2. アウトライン内でコメントスレッドを配置したい場所にカーソルを合わせるかタップし、紫の **+**（コンテンツを追加）ボタンをクリックします。
3. **Content Market** を選択します。Content Market パネルには、機関で承認されているすべての LTI ツールと Building Block 配置が一覧表示されます。
4. **FastComments** タイルを見つけてクリックします。Blackboard は **+** メニューを開いた位置にコンテンツ項目を作成します。
5. アイテムはデフォルトでアウトライン上に「学生に表示」エントリとして配置されます（講師個人のデフォルト設定で **Hide from students** がオフになっている場合）。デフォルトが **Hidden** の場合、アイテムは非表示で作成され、準備ができたらアイテム行の表示切替で可視にします。
6. アイテムの名前を変更するには、アウトラインでタイトルをクリックして新しいラベルを入力します。学生がアウトラインで見るタイトルは FastComments のスレッド識別子とは独立しているため、いつでも安全に名前を変更できます。

もし **Content Market** がオプションに表示されない場合は、機関がその配置を非表示にしています。同じ **+** メニューの **More tools** を通じて、**LTI Tools** グループ内から同じピッカーにアクセスできます。

#### Original Course View

Original Course View は Learn SaaS で引き続きサポートされており、Q4 2024 CU リリースライン上のセルフホスト型 Learn 9.1 サイトでは引き続き主要な体験です。

1. コースを開き、**Content Area**（例: コースメニューの既定の **Information** または **Content** エリア）に入ります。
2. 画面右上のトグルで **Edit Mode** をオンにします。
3. アクションバーで **Build Content** をクリックします。
4. **Learning Tools** サブメニューの下で **FastComments** をクリックします。Learning Tools サブメニューは、管理者がツールを登録した後の LTI 1.3 ツール配置から構成されます。ここに表示されない場合は、以下の注意点セクションを参照してください。
5. **Create FastComments** フォームで次を設定します:
   - **Name**: 学生がコンテンツエリアで見るラベル。
   - **Description**: 埋め込まれたスレッドの上に表示される任意のテキスト。
   - **Permit Users to View this Content**: 可用性の Yes/No トグル。
   - **Track Number of Views**: Blackboard のアイテム別閲覧統計を取得する場合に有効にします。FastComments は独自の解析を別途実行します。
   - **Date and Time Restrictions**: 任意の **Display After** / **Display Until** ウィンドウ。
6. 送信します。ツールがコンテンツエリアにクリック可能なアイテムとして表示されます。

#### Embedding Inside an Item or Document

両方のコースビューで、講師はコンテンツエディタの LTI Advantage ボタンを使って、Item、Document、または任意のリッチテキストフィールドの本文内に FastComments をインライン埋め込みできます。

Ultra Course View:

1. **Document** を作成または編集します。
2. スレッドを表示したいドキュメント本文内で **Add content** をクリックします。
3. エディタのツールバーで **Insert content** メニューを開き、**Content Market**（LTI Advantage / Deep Linking の入口）をクリックします。
4. **FastComments** を選びます。FastComments はディープリンクのペイロードを返し、Blackboard はカーソル位置に埋め込みブロックをドキュメント本文に挿入します。
5. ドキュメントを保存します。学生はスクロールしてその位置を通過すると、スレッドがインラインで表示されます。

Original Course View:

1. リッチテキスト本文を持つ任意のアイテムを編集します。
2. コンテンツエディタのツールバーで **Add Content** のプラスアイコンをクリックし、**Content Market**（古い Q4 2024 CU では **Add Content from External Tool** と表示）を選択します。
3. **FastComments** を選びます。エディタはディープリンクされたリソースを参照するプレースホルダブロックを挿入します。
4. アイテムを送信します。

各ディープリンク埋め込みはそれぞれ独立した FastComments スレッドを生成するため、1 つのアイテムに FastComments ブロックが 2 つ埋め込まれていれば、2 つの独立したコメントストリームになります。

#### Visibility, Release Conditions, and Group Restrictions

FastComments のコンテンツアイテムは、それらに適用されるアクセス制御ルールについて他の Blackboard コンテンツアイテムと同じように動作します。

- Ultra: 行の表示切替（**Visible to students**、**Hidden from students**、**Conditional availability**）をクリックします。Conditional availability は日付/時刻ウィンドウ、成績表アイテムに対するパフォーマンスルール、およびコースグループに対するメンバーシップルールをサポートします。
- Original: アイテムのコンテキストメニューを開き **Adaptive Release** または **Adaptive Release: Advanced** を選択して、日付、メンバーシップ、成績、またはレビュー状況でツールを制限します。アイテムで **Set Group Availability** を使用して特定のコースグループに制限します。

FastComments は Blackboard のゲート設定をそのまま尊重します。Blackboard が学生にアイテムを非表示にした場合、その学生には LTI ランチは発生せず、モデレータビューに表示されません。

#### Gradebook Behavior

FastComments は LTI Advantage の Assignment and Grade Services を通じて成績を報告しません。FastComments コンテンツアイテムに対して自動的に成績列は作成されません。

ただし、Blackboard テナントが評価メタデータに関係なくすべての新しいコンテンツアイテムに対して成績列を自動作成するよう設定されている場合、空の列が表示されます。非表示にするには:

- Ultra: **Gradebook** を開き、列ヘッダをクリックして **Edit** を選び、**Show to students** と **Include in calculations** をオフにします。あるいは、機関が未採点アイテムの列削除を許可していれば **Delete** を使用します。
- Original: **Grade Center** を開き、列のシェブロンをクリックして **Hide from Users (on/off)** を選び、オプションで **Column Organization** の下にある **Hide from Instructor View** を使用します。

#### What Students See

学生が FastComments アイテムを開くか埋め込まれたブロックまでスクロールすると:

1. Blackboard が FastComments に LTI 1.3 メッセージを送信します。学生は Blackboard の識別情報（名前、メール、アバター、役割）を使用して SSO によってサインインされ、ログインフォームは表示されません。
2. コメントスレッドが iframe 内にレンダリングされます。スレッド機能、返信、メンション、リアクションは、FastComments のコメントウィジェット設定に基づいて利用可能です。
3. 学生のコメントは Blackboard アカウントに紐づけられます。後で学生が Blackboard で名前や写真を編集すると、次回のランチ時に FastComments のプロフィールが更新されます。

Blackboard から FastComments へのロールマッピング:

- **System Administrator** と **Course Builder** は FastComments の **admin** にマップされます。
- **Instructor** と **Teaching Assistant** は FastComments の **moderator** にマップされます。
- **Student**、**Guest**、および **Observer** は FastComments の **commenter** にマップされます。

モデレータは各コメントに対してインラインでモデレーションコントロール（ピン、非表示、禁止、削除）を表示します。

#### Thread Scoping

FastComments は各スレッドを **(Blackboard host, course ID, resource link ID)** でスコープします。同じコース内の 2 つの FastComments アイテムは 2 つのスレッドを生成します。同一アイテムが 2 つのコースシェルにコピーされると（例えばコースコピーを通じて）、Blackboard がコピー時に新しい resource link ID を発行するため 2 つのスレッドが生成されます。コースコピー間で共有スレッドを維持するには、コピーを実行する前に FastComments で明示的なスレッド URN を設定した Deep Linking を使用してください。

#### Blackboard-Specific Gotchas

**Build Content メニュー（Original）または Content Market（Ultra）に FastComments タイルがない。** 管理者はツールを承認したが、該当する配置をブロックする機関ポリシーを残している可能性があります。**Administrator Panel** > **Integrations** > **LTI Tool Providers** に移動し、FastComments エントリを編集して、**Course Content Tool**（Original）および **Course Content Tool - allow students** / **Deep Linking content tool**（Ultra）の配置が有効になっていることを確認します。保存してコースページを更新してください。

**起動時に「Tool not configured for this context」または「Tool is not deployed」エラーが出る。** 動的登録時に登録されたデプロイメントスコープがコースの属する機関コンテキストと一致していません。Blackboard のツールプロバイダエントリで **Deployment ID** が、このテナントの FastComments の LTI 1.3 Configuration ページに表示されているものと一致するか確認してください。異なる場合は、配置を削除して新しい登録 URL から動的登録を再実行します。

**Iframe の高さが固定されているように見える、またはコンテンツが切れる。** 一部の Blackboard テナントはデフォルトの LTI iframe リサイズの postMessage をブロックする厳格な Content Security Policy を導入していることがあります。FastComments は互換性を最大化するために Canvas スタイルの `lti.frameResize` メッセージと IMS 仕様形式の `org.imsglobal.lti.frameResize` メッセージの両方を送出しますが、テナントレベルの CSP オーバーライドが親側のリスナーをブロックしている場合があります。管理者に `*.fastcomments.com` が LTI ツールの許可リストに含まれていること、カスタム CSP ヘッダが postMessage イベントを削除していないことを確認するよう依頼してください。そうすればリサイズは追加設定なしで動作します。

**コースコピーでスレッドが複製される。** Blackboard のコースコピーは LTI 配置に対して新しい resource link ID を発行するため、コピーされたコースは空のスレッドで開始します。これは想定された動作です。コピー先のコースが元のスレッドを継承する必要がある場合は、コピー前に明示的なスレッド URN を設定した Deep Linking を設定するか、FastComments サポートに連絡してスレッド ID の一括リマップを依頼してください。

**学生が起動時に一般的な Blackboard エラーを見る。** 原因は欠落または古い `email` クレームです。FastComments の機関ポリシーで **User Fields to Send** の下に **Role**、**Name**、および **Email Address** が有効になっていることを確認してください。保存後、新しいブラウザセッションで再度起動してください。