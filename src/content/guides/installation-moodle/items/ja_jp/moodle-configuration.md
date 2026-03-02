プラグイン設定ページは **サイト管理 > プラグイン > ローカルプラグイン > FastComments** にあります。利用可能なオプションは次のとおりです：

#### Tenant ID

あなたの FastComments テナント ID。アカウント設定の <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments ダッシュボード</a> で確認できます。

#### API Secret

Secure SSO モードで必要な API Secret キーです。<a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">マイアカウント > API シークレット</a> で確認できます。

#### SSO Mode

ユーザーがどのように認証されるかを選択します。各オプションの詳細は [SSO モード](#moodle-sso-modes) セクションを参照してください。

- **Secure**（推奨） - サーバー側で HMAC-SHA256 による署名付き認証
- **Simple** - 署名なしのクライアント側ユーザーデータ
- **None** - 匿名コメント（Moodle ログインの統合なし）

#### Page Contexts

コメントを表示する場所を制御します：

- **Course pages** - コースのメインページにコメント
- **Module/activity pages** - 個々のアクティビティやリソースのページにコメント
- **Both** - すべてのページタイプにコメント

#### Commenting Style

コメントの操作方法を選択します。各モードのスクリーンショットは [コメントスタイル](#moodle-commenting-styles) を参照してください。

- **Comments** - ページ内容の下に表示される標準的なスレッド型コメントウィジェット
- **Collab Chat** - テキスト選択によるインラインのディスカッション（プレゼンス表示あり）
- **Both** - コメントと Collab Chat の両方が有効

#### CDN URL

FastComments の CDN URL。デフォルトは `https://cdn.fastcomments.com`。データが EU リージョンにホストされている場合は、これを EU の CDN URL に変更してください。