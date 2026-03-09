#### CanvasでDeveloper Keysを開く

管理者としてCanvasにログインします。左サイドバーの**Admin**へ移動 > アカウントを選択 > **Developer Keys**。

#### LTI Developer Keyを作成する

「**+ Developer Key**」をクリックし、**LTI Key**を選択します。

設定フォームで:

1. 左側の**Redirect URIs**フィールドに、FastCommentsのセットアップページからコピーした**Launch URL**を貼り付けます。
2. 右側で**Method**を**Enter URL**に設定します。
3. FastCommentsからコピーした**Configuration URL**を**JSON URL**フィールドに貼り付けます。
4. CanvasがLTI構成を自動的に読み込みます。
5. キーに名前を付けます（例: "FastComments"）。
6. 「**Save**」をクリックします。

#### Developer Keyを有効にする

保存後、新しいキーがDeveloper Keysテーブルに表示され、**State**が**OFF**になっています。トグルをクリックして**ON**に切り替えます。Canvasが確認を求める場合は、**Allow**をクリックしてキーを有効にします。

#### Client IDをコピーする

Developer KeysテーブルのDetails列に数値の**Client ID**が表示されます（例: `17000000000042`）。この番号をコピーしてください — 次の手順で必要になります。