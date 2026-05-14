このページでは、管理者がツールを登録してデプロイメントを作成した後に、Brightspace コースへ FastComments を追加する方法を説明します。ツールがまだ登録されていない場合は、最初に D2L 登録ガイドを参照してください。

<div class="screenshot white-bg">
    <div class="title">Brightspace のユニットトピックに埋め込まれた FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="Brightspace ユニット内で動作する FastComments。スレッド化されたコメントと @メンションピッカーを表示しています" />
</div>

Brightspace には 2 つのコンテンツ作成体験が搭載されています：**Classic Content** と **New Content Experience**（**Lessons** とも呼ばれます）。どちらも FastComments を公開していますが、メニュー経路が異なります。以下の各セクションは、分岐する箇所で両方を取り上げます。

#### FastComments ツールを見つける

FastComments ツールは、コースのコンテンツエディタ内で次の 2 か所に表示されます：

1. モジュール／ユニットの **Add Existing** ボタン（古い Brightspace では **Add Existing Activities** と表示）からアクセスするアクティビティピッカー。最新の Brightspace では FastComments はピッカーに直接表示されます。古いバージョンでは **External Learning Tools** サブメニューの下にネストされています。どちらの経路でも FastComments を単独のトピックとして追加します。
2. HTML エディタ内の **Insert Stuff** ダイアログの **LTI Advantage** の下。これは LTI ディープリンクフローを通じて FastComments を HTML トピック内にインラインで埋め込みます。

FastComments がどちらのピッカーにも表示されない場合、そのコースを含む組織ユニットに対してデプロイメントが有効になっていません。Brightspace 管理者に依頼して **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments** を開き、デプロイメントを開いて **Org Units** の下にコースの org unit（または親の org unit）を追加してもらってください。

#### モジュールにトピックとして FastComments を追加する

Classic Content:

1. コースを開き、ナビバーの **Content** をクリックします。
2. ディスカッションを含めるモジュールを選択する（または **Add a module** で作成）。
3. **Add Existing** をクリックします（古い Brightspace：**Add Existing Activities** > **External Learning Tools**）。
4. ピッカーで **FastComments** をクリックします。Brightspace がモジュール内にトピックを作成し、コンテンツビューに戻ります。
5. 新しいトピックをクリックし、インラインタイトルエディタで説明的な名前（例：`FastComments Discussion`）に変更します。

New Content Experience (Lessons):

1. コースを開き、**Content** をクリックします。
2. ディスカッションを含めるユニットとレッスンを開きます。
3. **Add** > **Existing Activity** をクリックし、**FastComments** を選択します（古い Brightspace：**External Learning Tools** の下にネスト）。
4. アクティビティがレッスンに追加されます。
5. アクティビティのタイトルをクリックして名称を変更します。

初めてユーザー（講師または学生を問わず）がトピックを開くと、FastComments がそのリソースリンクのスレッドを初期化します。スレッドはリソースリンク ID に紐付けられるため、トピックの名前変更や移動は読み込まれるスレッドを変更しません。

#### HTML トピックに FastComments をインラインで埋め込む

読み物、ビデオ、その他のコンテンツの下に同じトピックページ内でコメントを表示したい場合は、このフローを使用します。別トピックとしてではなく、同一ページ内に表示されます。

1. モジュール／レッスンで HTML トピックを開くか作成します。
2. **Edit HTML** をクリックして Brightspace の HTML エディタを開きます。
3. コメントスレッドを表示したい箇所にカーソルを置きます。
4. エディタツールバーの **Insert Stuff** ボタン（パズルピースアイコン）をクリックします。
5. Insert Stuff ダイアログで **LTI Advantage** をスクロールして **FastComments** をクリックします。
6. FastComments がディープリンクピッカーを開きます。配置を確認します（コンテンツ用ディスカッションではデフォルトオプションで問題ありません）；**Insert** または **Continue** をクリックします。
7. Brightspace はプレースホルダーブロックを伴って HTML エディタに戻します（LTI 起動を表す）。トピックで **Save and Close** をクリックします。

トピックが読み込まれると、Brightspace はプレースホルダーを iframe に置き換え、LTI 経由で FastComments を自動起動します。学生はディスカッションスレッドをインラインで見ます。

1 つの HTML トピックには複数のディープリンクされた FastComments 埋め込みを保持できます。各埋め込みは別個のリソースリンク ID を生成するため、それぞれに独自のスレッドが割り当てられます。

#### モジュールトピックとインラインクイックリンクの使い分け

次の場合は**モジュールトピック**方式を選んでください：

- そのモジュールのステップにおける主要なアクティビティがディスカッションである場合。
- Brightspace の目次、完了トラッキング、Class Progress にトピックを表示したい場合。

次の場合は**インライン埋め込み**方式を選んでください：

- コメントを同一ページ上の他のコンテンツの下に置きたい場合。
- 目次に別項目として表示される完了トラッキング対象アイテムを望まない場合。

#### 表示、下書き、リリース条件

新しい FastComments トピックはデフォルトで学生に表示されます。セットアップ中に非表示にするには：

1. コンテンツエディタでトピックタイトル（Classic）またはアクティビティの三点メニュー（New Content Experience）をクリックします。
2. ステータスを **Draft** に設定する（Classic）か、**Visibility** をオフに切り替える（New Content Experience）。

下書きトピックは学生には見えません。講師と TA は「Draft」バッジ付きでそれらを引き続き見ることができます。

トピックを特定のグループやセクションに限定するには：

1. トピックを開きます。
2. トピックタイトルメニュー > **Edit Properties In-place**（Classic）または **Edit** > **Restrictions**（New Content Experience）をクリックします。
3. **Release Conditions** の下で **Create** をクリックします。
4. **Group enrollment** または **Section enrollment** を選び、グループ／セクションを選択して保存します。

リリース条件は FastComments の独自のロールマッピングと組み合わされます。トピックを表示できない学生は LTI 起動を受けません。

#### 学生が最初に起動したときに見るもの

学生がトピックをクリックしたとき（または埋め込みのある HTML トピックを読み込んだとき）：

1. Brightspace がバックグラウンドで LTI 1.3 起動を実行します。
2. FastComments は学生の名前、メール、アバター URL、および LMS のロールを受け取り、自動的にサインインさせます。FastComments のログイン画面は表示されません。
3. そのリソースリンクのコメントスレッドが Brightspace の iframe 内にレンダリングされます。

起動時のロールマッピング：

- Brightspace `Administrator` はスレッドの FastComments の管理者（admin）になります（完全なモデレーション、削除、禁止、設定へのアクセス）。
- Brightspace `Instructor` は FastComments のモデレーター（moderator）になります（ピン、非表示、削除、禁止）。
- その他すべてのロール（`Learner`、`TeachingAssistant` など）は通常のコメント投稿者になります。

コメントは学生の Brightspace アカウントに紐付けられます。学生が Brightspace 側で名前やアバターを編集すると、次回の LTI 起動で変更が同期されます。

#### iframe の高さとリサイズ

FastComments はスレッドのレンダリングごとおよびコンテンツの変更時（新しいコメント、返信の展開など）に `org.imsglobal.lti.frameResize` の postMessage を送信します。Brightspace はこのメッセージをリッスンし、スレッドが切り取られず内部スクロールバーが表示されないように iframe の高さを調整します。

iframe が固定で短い高さのままになる場合：

- コースが HTTPS 経由で読み込まれていることを確認してください。Brightspace の postMessage リスナーは混在コンテンツのフレームを拒否します。
- ブラウザ拡張機能が postMessage チャネルをブロックしていないことを確認してください。
- HTML トピック内のインライン埋め込みの場合、周囲の HTML が iframe を固定高さのコンテナでラップしていないことを確認してください。親要素の inline な style="height: ..." を削除してください。

#### Brightspace 固有の注意点

**Add Existing ピッカーにツールが表示されない。** デプロイメントがこのコースの org unit に対して有効になっていません。管理者がデプロイメントの **Org Units** リストに org unit（または親）を追加する必要があります。ツールの登録だけでは不十分で、デプロイメントがどのコースにツールを見せるかを決定します。

**`deployment_id` の不一致による起動エラー。** FastComments は登録時に最初に見た `deployment_id` を TOFU（初回見たら固定）します。管理者が元のデプロイメントを削除して新しいものを作成した場合、新しいデプロイメントからの起動はデプロイメント不一致エラーで拒否されます。対処法は FastComments を再登録することです（新しい登録 URL を生成して動的登録を再実行してください）；古い設定レコードは置き換えられます。

**ツールは起動するが「Invalid LTI launch」と表示される。** コースがデプロイメントがカバーするテナント／組織構造と異なるか、登録後にデプロイメントが無効化された可能性があります。**Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** トグルと、デプロイメントの org unit リストを再確認してください。

**FastComments 内で名前やロールが欠落している。** Brightspace は Names and Role Provisioning Services (NRPS) クレーム付きで LTI 起動を送付します。コースが古い LTI 1.1 リンクからアップグレードされた場合、起動に `name` および `email` クレームが欠けていることがあります。FastComments トピックを **Add Existing** で再追加してください（古いリンクを移行しない）；そうすることで起動が LTI 1.3 を使用するようになります。

**埋め込みが自動 SSO ではなくログイン画面を表示する。** HTML トピックが **Insert Stuff** > **LTI Advantage** を使わずに FastComments を指す単純な `<iframe>` として挿入されている可能性があります。単純な iframe は LTI 起動をスキップし、ユーザーを公開向けの FastComments ページに誘導します。iframe を削除し、Insert Stuff のフローで再挿入してください。