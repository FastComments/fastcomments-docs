#### Canvasで開発者キーを開く

管理者としてCanvasにログインします。左のサイドバーで**Admin**を開き、アカウントを選択して**Developer Keys**に移動します。

#### LTI Developer Keyを作成する

**+ Developer Key** をクリックし、**LTI Key** を選択します。

設定フォームで:

1. **Method** を **Enter URL** に設定します。
2. FastCommentsからコピーした**Configuration URL**をURL欄に貼り付けます。
3. CanvasがLTI設定を自動的に読み込みます。
4. キーに名前を付けます（例: "FastComments"）。
5. **Save** をクリックします。

#### Developer Keyを有効にする

保存後、新しいキーがDeveloper Keysのテーブルに表示され、**State** が **OFF** に設定されています。トグルをクリックして **ON** に切り替えます。Canvasが確認を求める場合は、**Allow** をクリックしてキーを有効にします。

#### クライアントIDをコピーする

Developer KeysのテーブルのDetails列に数値の**Client ID**が表示されます（例: `17000000000042`）。この番号をコピーしてください — 次のステップで必要になります。