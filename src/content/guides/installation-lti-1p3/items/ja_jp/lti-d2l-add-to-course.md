このページでは、管理者がツールを登録しデプロイメントを作成した後に、Brightspace コースに FastComments を追加する手順を説明します。ツールがまだ登録されていない場合は、まず D2L 登録ガイドを参照してください。

<div class="screenshot white-bg">
    <div class="title">Brightspace のユニットトピックに埋め込まれた FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="Brightspace のユニット内で動作する FastComments。スレッド化されたコメントと @ メンションピッカーを表示しています" />
</div>

Brightspace には 2 つのコンテンツ作成体験が含まれます: **Classic Content（クラシックコンテンツ）** と **New Content Experience（新しいコンテンツ体験、**Lessons** とも呼ばれる）**。どちらも FastComments を利用できますが、メニューのパスが異なります。以下の各セクションでは、分岐する箇所で両方を扱います。

#### FastComments ツールの場所を特定する

FastComments ツールは、コースのコンテンツエディタ内で次の 2 か所に表示されます:

1. モジュール/ユニットの **Add Existing** ボタン（古い Brightspace では **Add Existing Activities** と表示）からアクセスするアクティビティピッカー。現在の Brightspace ではピッカー内に直接 FastComments が表示されます。古いバージョンでは **External Learning Tools** サブメニューの下にネストされています。どちらの経路でも FastComments を独立したトピックとして追加します。
2. HTML エディタ内の **Insert Stuff** ダイアログの **LTI Advantage** の下。これにより LTI のディープリンクフローを通じて HTML トピック内に FastComments をインライン埋め込みできます。

どちらのピッカーにも FastComments が表示されない場合は、デプロイメントがそのコースを保持する組織ユニットに対して有効になっていません。Brightspace 管理者に依頼して、**Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments ツール > **View Deployments** を開き、デプロイメントを開いて **Org Units** の下にコースの組織ユニット（または親の組織ユニット）を追加してもらってください。

#### モジュールにトピックとして FastComments を追加する

Classic Content（クラシックコンテンツ）:

1. コースを開き、ナビバーの **Content** をクリックします。
2. ディスカッションを格納するモジュールを選択します（または **Add a module** で作成します）。
3. **Add Existing** をクリックします（古い Brightspace: **Add Existing Activities** > **External Learning Tools**）。
4. ピッカーで **FastComments** をクリックします。Brightspace がモジュールにトピックを作成し、コンテンツビューに戻ります。
5. 新しいトピックをクリックします。インラインタイトルエディタを使って、`FastComments Discussion` のような説明的な名前に変更します。

New Content Experience（Lessons）:

1. コースを開き、**Content** をクリックします。
2. ディスカッションを格納するユニットとレッスンを開きます。
3. **Add** > **Existing Activity** をクリックし、**FastComments** を選択します（古い Brightspace: **External Learning Tools** の下にネスト）。
4. アクティビティがレッスンに追加されます。
5. アクティビティのタイトルをクリックして名前を変更します。

どのユーザー（講師または学生）でもトピックを初めて開くと、FastComments はそのリソースリンク用のスレッドを初期化します。スレッドはリソースリンク ID に紐付けられるため、トピックの名前を変更したり移動したりしても読み込まれるスレッドは変わりません。

#### HTML トピック内に FastComments をインライン埋め込む

読み物、ビデオ、またはその他のコンテンツの下に同じトピックページ内でコメントを表示したい場合は、このフローを使用します（別トピックとしてではなく）。

1. モジュール/レッスンで HTML トピックを開くか作成します。
2. **Edit HTML** をクリックして Brightspace の HTML エディタを開きます。
3. コメントスレッドを表示したい位置にカーソルを置きます。
4. エディタツールバーのパズルピースアイコン（**Insert Stuff** ボタン）をクリックします。
5. Insert Stuff ダイアログで **LTI Advantage** にスクロールし、**FastComments** をクリックします。
6. FastComments がディープリンクピッカーを開きます。配置を確認します（デフォルトオプションはコンテンツディスカッションに適しています）。**Insert** または **Continue** をクリックします。
7. Brightspace はプレースホルダーブロックを伴って HTML エディタに戻します（LTI ランチを表すもの）。トピックで **Save and Close** をクリックします。

トピックが読み込まれると、Brightspace はプレースホルダーを iframe に置き換え、LTI 経由で自動的に FastComments を起動します。学生はインラインでディスカッションスレッドを見ることができます。

単一の HTML トピックには複数のディープリンクされた FastComments 埋め込みを保持できます。各埋め込みはそれぞれ固有のスレッドを持ちます。各ディープリンクは異なるリソースリンク ID を生成するためです。

#### モジュールトピック vs インライン クイックリンク

次の場合は「モジュールトピック」アプローチを選んでください:

- そのモジュールのステップにおいてディスカッションが主要なアクティビティである場合。
- Brightspace の目次、完了トラッキング、および Class Progress にトピックを表示したい場合。

次の場合は「インライン埋め込み」アプローチを選んでください:

- コメントを同じページ内の他のコンテンツの下に配置したい場合。
- 目次に別の完了トラッカブルな項目を表示したくない場合。

#### 可視性、ドラフト、リリース条件

新しい FastComments トピックはデフォルトで学生に表示されます。セットアップ中に非表示にするには:

1. コンテンツエディタでトピックタイトルをクリックします（Classic）またはアクティビティの三点メニューをクリックします（New Content Experience）。
2. ステータスを **Draft** に設定する（Classic）か、**Visibility** をオフに切り替えます（New Content Experience）。

ドラフトトピックは学生には表示されません。講師と TA は「Draft」バッジ付きで引き続き表示できます。

特定のグループやセクションにトピックを制限するには:

1. トピックを開きます。
2. トピックタイトルメニュー > **Edit Properties In-place**（Classic）または **Edit** > **Restrictions**（New Content Experience）をクリックします。
3. **Release Conditions** の下で **Create** をクリックします。
4. **Group enrollment** または **Section enrollment** を選択し、グループ/セクションを指定して保存します。

リリース条件は FastComments の独自のロールマッピングと組み合わされます。トピックを表示できない学生は LTI ランチを受け取りません。

#### 学生の初回起動時に表示されるもの

学生がトピックをクリックする（または埋め込みのある HTML トピックを読み込む）と:

1. Brightspace がバックグラウンドで LTI 1.3 のランチを実行します。
2. FastComments は学生の名前、メール、アバター URL、LMS ロールを受け取り、自動的にサインインします。FastComments のログイン画面は表示されません。
3. そのリソースリンク用のコメントスレッドが Brightspace の iframe 内にレンダリングされます。

ランチ時のロールマッピング:

- Brightspace の `Administrator` はスレッドに対して FastComments の管理者（admin）になります（完全なモデレーション、削除、追放、設定アクセス）。
- Brightspace の `Instructor` は FastComments のモデレーター（moderator）になります（ピン、非表示、削除、追放）。
- その他すべてのロール（`Learner`、`TeachingAssistant` など）は標準のコメント投稿者になります。

コメントは学生の Brightspace アカウントに帰属します。学生が Brightspace で名前やアバターを編集した場合、次回の LTI ランチで変更が同期されます。

#### 公開アクセスを制限する（推奨）

デフォルトでは、FastComments のコメントデータは公開で読み取り可能です。スレッドの URL や API エンドポイントを推測できる人は、Brightspace 以外からでもコメントを閲覧できます。コースのディスカッションでは、履修学生のみに閲覧を制限するのがほとんどの場合望ましいです。

自分の <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> を開き、**Require SSO To View Comments** が有効なルールを作成し、セキュリティレベルを **Secure SSO** に設定します。これによりスレッドは署名付きの LTI ランチを通じてのみ読み込めるようになります。

スレッドを単一ドメインやページにスコープする方法など、完全な手順は [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) を参照してください。

#### iframe の高さとリサイズ

FastComments はスレッドのレンダリング時およびコンテンツ変更時（新しいコメント、返信の展開など）に `org.imsglobal.lti.frameResize` の postMessage を送信します。Brightspace はこのメッセージを受信して iframe の高さを調整し、スレッドが切り取られたり内部でスクロールバーが表示されたりしないようにします。

iframe の高さが固定された短いままの場合:

- コースが HTTPS 経由で読み込まれていることを確認してください。Brightspace の postMessage リスナーは混在コンテンツのフレームを拒否します。
- ブラウザ拡張機能が postMessage チャネルをブロックしていないことを確認してください。
- HTML トピック内のインライン埋め込みの場合、親の HTML が iframe を固定高さのコンテナでラップしていないことを確認してください。親要素の inline な `style="height: ..."` を削除してください。

#### Brightspace 固有のトラブルシューティング

**Add Existing ピッカーにツールが表示されない。** デプロイメントがこのコースの組織ユニットに対して有効になっていません。管理者がデプロイメントの **Org Units** リストに組織ユニット（または親）を追加する必要があります。ツールの登録だけでは不十分です。デプロイメントがどのコースがツールを表示できるかを制御します。

**ランチ時の `deployment_id` の不一致。** FastComments は登録時に最初に見た `deployment_id` を TOFU 的にピン留めします。管理者が元のデプロイメントを削除して新しいものを作成した場合、新しいデプロイメントからのランチはデプロイメント不一致エラーで拒否されます。修正方法は FastComments を再登録することです（新しい登録 URL を生成して（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ここで取得できます</a>）Dynamic Registration を再実行します）；古い構成レコードは置き換えられます。

**ツールは起動するが「Invalid LTI launch」と表示される。** コースがデプロイメントがカバーするテナント/組織構造と異なる、または登録後にデプロイメントが無効化された可能性があります。**Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** トグルとデプロイメントの組織ユニットリストを再確認してください。

**FastComments 内で名前やロールが欠落している。** Brightspace は Names and Role Provisioning Services（NRPS）クレームを含む LTI ランチを送信します。コースが古い LTI 1.1 リンクからアップグレードされた場合、ランチに `name` と `email` クレームが欠けていることがあります。FastComments トピックを再度 **Add Existing** で追加してください（古いリンクを移行しないでください）。そうすることでランチが LTI 1.3 を使用します。

**埋め込みが自動 SSO ではなくログイン画面を表示する。** HTML トピックが **Insert Stuff** > **LTI Advantage** を経由せず、FastComments を直接指す通常の `<iframe>` として挿入されている可能性があります。通常の iframe は LTI ランチをスキップし、ユーザーを公開向けの FastComments ページに送ります。iframe を削除し、Insert Stuff フローで再挿入してください。