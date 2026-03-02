プラグイン設定ページは **サイト管理 > プラグイン > ローカルプラグイン > FastComments** にあります。利用可能なオプションは次の通りです：

#### テナント ID

FastComments のテナント ID。アカウント設定内の <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments ダッシュボード</a> で確認できます。

#### API Secret

Secure SSO モードで必要な API Secret キーです。これは <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a> で確認できます。

#### SSO モード

ユーザーがどのように認証されるかを選択します。各オプションの詳細は [SSO モード](#items-moodle-sso-modes) セクションを参照してください。

- **Secure**（推奨） - サーバー側で HMAC-SHA256 による署名付き認証
- **Simple** - 署名のないクライアント側のユーザーデータ
- **None** - 匿名コメント、Moodle ログイン統合なし

#### ページコンテキスト

コメントが表示される場所を制御します：

- **コースページ** - コースのメインページに表示されるコメント
- **モジュール/アクティビティページ** - 個別のアクティビティやリソースに対するコメント
- **両方** - すべてのページタイプにコメントを表示

#### コメントスタイル

コメントの体験方法を選択します。各モードのスクリーンショットは [コメントスタイル](#items-moodle-commenting-styles) を参照してください。

- **Comments** - ページコンテンツの下に表示される標準的なスレッド式コメントウィジェット
- **Collab Chat** - テキスト選択に基づくインラインディスカッション（プレゼンス表示付き）
- **Both** - コメントとコラボチャットの両方を同時に有効化

#### CDN URL

FastComments の CDN URL。デフォルトは `https://cdn.fastcomments.com` です。データが EU リージョンでホストされている場合は、これを EU の CDN URL に変更してください。

---