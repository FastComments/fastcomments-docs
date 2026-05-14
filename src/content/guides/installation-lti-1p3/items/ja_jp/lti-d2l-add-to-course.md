このページでは、管理者がツールを登録しデプロイメントを作成した後に、Brightspace コースに FastComments を追加する方法を説明します。ツールがまだ登録されていない場合は、まず D2L 登録ガイドを参照してください。

<div class="screenshot white-bg">
    <div class="title">Brightspace のユニットトピックに埋め込まれた FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="Brightspace ユニット内で動作している FastComments。スレッド化されたコメントと @-メンションピッカーを表示しています" />
</div>

Brightspace には 2 つのコンテンツ作成エクスペリエンスが搭載されています: **Classic Content** と **New Content Experience**（**Lessons** とも呼ばれます）。どちらも FastComments を利用できますが、メニューの経路が異なります。以下の各セクションでは、分岐する箇所で両方をカバーします。

#### FastComments ツールを見つける

FastComments ツールはコースのコンテンツエディタ内の 2 箇所に表示されます:

1. モジュール／ユニットの **Add Existing** ボタン（古い Brightspace では **Add Existing Activities** と表示）からアクセスするアクティビティピッカー。現在の Brightspace ビルドでは FastComments はピッカーに直接表示されますが、古いバージョンでは **External Learning Tools** サブメニューの下にネストされています。どちらの経路でも FastComments を単独のトピックとして追加します。
2. HTML エディタ内の **Insert Stuff** ダイアログの **LTI Advantage**。これは LTI のディープリンクフローを介して HTML トピック内に FastComments をインラインで埋め込みます。

どちらのピッカーにも FastComments が表示されない場合、デプロイメントがそのコースを含む組織ユニットで有効になっていません。Brightspace 管理者に **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments ツール > **View Deployments** を開いてデプロイメントを開き、**Org Units** の下にコースの組織ユニット（または親の組織ユニット）を追加するよう依頼してください。

#### モジュールに FastComments をトピックとして追加する

Classic Content:

1. コースを開き、ナビバーで **Content** をクリックします。
2. ディスカッションを置くモジュールを選択します（または **Add a module** で作成します）。
3. **Add Existing** をクリックします（古い Brightspace: **Add Existing Activities** > **External Learning Tools**）。
4. ピッカーで **FastComments** をクリックします。Brightspace はモジュールにトピックを作成し、コンテンツ表示に戻ります。
5. 新しいトピックをクリックします。インラインタイトルエディタを使って `FastComments Discussion` のようなわかりやすい名前に変更します。

New Content Experience (Lessons):

1. コースを開き、**Content** をクリックします。
2. ディスカッションを置くユニットとレッスンを開きます。
3. **Add** > **Existing Activity** をクリックし、**FastComments** を選択します（古い Brightspace: **External Learning Tools** の下にネストされています）。
4. アクティビティがレッスンに追加されます。
5. アクティビティタイトルをクリックして名前を変更します。

最初に誰か（講師または学生）がトピックを開くと、FastComments はそのリソースリンク用のスレッドを初期化します。スレッドはリソースリンク ID に紐づくため、トピック名の変更や移動は読み込まれるスレッドを変更しません。

#### HTML トピック内に FastComments をインラインで埋め込む

読み物、ビデオ、または同じトピックページ内の他のコンテンツの下にコメントを表示したい場合は、このフローを使用してください。別トピックではなく同一ページ内に表示されます。

1. モジュール／レッスンで HTML トピックを開くか作成します。
2. Brightspace の HTML エディタで **Edit HTML** をクリックして開きます。
3. コメントスレッドを表示したい位置にカーソルを置きます。
4. **Insert Stuff** ボタン（エディタツールバーのパズルピースアイコン）をクリックします。
5. Insert Stuff ダイアログで **LTI Advantage** にスクロールし、**FastComments** をクリックします。
6. FastComments はディープリンクピッカーを開きます。配置を確認します（コンテントディスカッションにはデフォルトオプションで問題ありません）。**Insert** または **Continue** をクリックします。
7. Brightspace はプレースホルダブロックを伴って HTML エディタに戻します。トピックで **Save and Close** をクリックします。

トピックがロードされると、Brightspace はプレースホルダを iframe に置き換え、LTI を介して自動的に FastComments を起動します。学生はディスカッションスレッドをインラインで閲覧できます。

1 つの HTML トピックに複数のディープリンクされた FastComments 埋め込みを含めることができます。各埋め込みはそれぞれ異なるリソースリンク ID を生成するため、個別のスレッドを持ちます。

#### モジュールトピックとインラインクイックリンクの使い分け

次の場合は **モジュールトピック** のアプローチを選んでください:

- そのステップでの主要なアクティビティがディスカッションである場合。
- トピックを Brightspace の目次、完了トラッキング、および Class Progress に表示させたい場合。

次の場合は **インライン埋め込み** のアプローチを選んでください:

- コメントを同じページ上の他のコンテンツの下に表示したい場合。
- 目次に別個の完了トラッキング対象項目として表示させたくない場合。

#### 表示、ドラフト、およびリリース条件

新しい FastComments トピックはデフォルトで学生に表示されます。設定中に非表示にしたい場合:

1. コンテンツエディタでトピックタイトルをクリック（Classic）するか、アクティビティの三点メニュー（New Content Experience）をクリックします。
2. Classic ではステータスを **Draft** に設定、New Content Experience では **Visibility** をオフに切り替えます。

ドラフトのトピックは学生には表示されません。講師と TA は「Draft」バッジ付きでそれらを引き続き見ることができます。

トピックを特定のグループやセクションに制限するには:

1. トピックを開きます。
2. トピックタイトルメニュー > **Edit Properties In-place**（Classic）または **Edit** > **Restrictions**（New Content Experience）をクリックします。
3. **Release Conditions** の下で **Create** をクリックします。
4. **Group enrollment** または **Section enrollment** を選び、グループ／セクションを選択して保存します。

リリース条件は FastComments の独自のロールマッピングと組み合わされます。トピックを表示できない学生には LTI ランチは提供されません。

#### 学生が最初に起動したときに見るもの

学生がトピックをクリックした（または埋め込みのある HTML トピックを読み込んだ）とき:

1. Brightspace はバックグラウンドで LTI 1.3 ランチを実行します。
2. FastComments は学生の名前、メール、アバター URL、および LMS ロールを受け取り、自動的にサインインさせます。FastComments のログインプロンプトは表示されません。
3. そのリソースリンク用のコメントスレッドが Brightspace の iframe 内にレンダリングされます。

ランチ時のロールマッピング:

- Brightspace の `Administrator` はスレッドに対する FastComments の管理者（admin）になります（完全なモデレーション、削除、バン、および設定アクセス）。
- Brightspace の `Instructor` は FastComments のモデレーター（moderator）になります（ピン、非表示、削除、バン）。
- その他のすべてのロール（`Learner`、`TeachingAssistant` など）は標準のコメント投稿者になります。

コメントは学生の Brightspace アカウントに紐づけられます。学生が Brightspace 側で名前やアバターを編集した場合、次回の LTI ランチで変更が同期されます。

#### iframe の高さとリサイズ

FastComments はスレッドのレンダリングごとおよびコンテンツ変更時（新しいコメント、返信の展開など）に `org.imsglobal.lti.frameResize` の postMessage を送信します。Brightspace はこのメッセージをリッスンし、iframe の高さを調整してスレッドが切り取られたり内部のスクロールバーが表示されたりしないようにします。

iframe が短い固定高さのままの場合:

- コースが HTTPS 経由で読み込まれていることを確認してください。Brightspace の postMessage リスナーは混在コンテンツのフレームを拒否します。
- ブラウザ拡張機能が postMessage チャネルをブロックしていないことを確認してください。
- HTML トピック内のインライン埋め込みの場合、周囲の HTML が iframe を固定高さのコンテナでラップしていないことを確認してください。親要素の inline `style="height: ..."` を削除してください。

#### Brightspace 固有の注意点

**Add Existing ピッカーにツールが表示されない。** デプロイメントがこのコースの組織ユニットで有効になっていません。管理者はデプロイメントの **Org Units** リストに組織ユニット（または親のユニット）を追加する必要があります。ツールの登録だけでは不十分で、デプロイメントがどのコースでツールを見せるかを制御します。

**ランチ時の `deployment_id` の不一致。** FastComments は登録時に最初に見た `deployment_id` を TOFU（Trust On First Use）で固定します。管理者が元のデプロイメントを削除して新しいものを作成した場合、新しいデプロイメントからのランチはデプロイメント不一致エラーで拒否されます。修正方法は FastComments を再登録することです（新しい登録 URL を生成して（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ここから取得</a>）ダイナミック登録を再実行してください）；これにより古い構成レコードは置き換えられます。

**ツールは起動するが「Invalid LTI launch」と表示される。** コースがデプロイメントのカバー範囲と異なるテナント／組織構造にあるか、登録後にデプロイメントが無効化された可能性があります。**Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** トグルとデプロイメントの組織ユニットリストを再確認してください。

**FastComments 内で名前やロールが欠落している。** Brightspace は Names and Role Provisioning Services (NRPS) クレームを含む LTI ランチを送信します。コースが古い LTI 1.1 リンクからアップグレードされている場合、ランチに `name` や `email` クレームが欠けることがあります。FastComments トピックを **Add Existing** で再追加してください（古いリンクをマイグレートしないでください）。そうすることでランチは LTI 1.3 を使用します。

**埋め込みが自動 SSO ではなくログイン画面を表示する。** HTML トピックが **Insert Stuff** > **LTI Advantage** を経由せずに、FastComments を指す単なる `<iframe>` として挿入されている可能性があります。プレーンな iframe は LTI ランチをスキップし、ユーザーを公開向けの FastComments ページに導きます。iframe を削除し、Insert Stuff フローで再挿入してください。