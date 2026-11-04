プラグインの設定ページは **Site Administration > Plugins > Local plugins > FastComments** にあります。利用可能なオプションは以下の通りです:

#### Tenant ID

あなたの FastComments Tenant ID。これは <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments ダッシュボード</a> のアカウント設定で確認できます。

#### API Secret

Secure SSO モードに必要な API Secret キー。<a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a> で確認できます。

#### SSO Mode

ユーザーの認証方法を選択します。各オプションの詳細は [SSO Modes](#moodle-sso-modes) セクションを参照してください。

- **Secure**(推奨)- サーバーサイドで HMAC-SHA256 署名された認証
- **Simple** - 署名なしのクライアントサイドユーザーデータ
- **None** - 匿名コメント。Moodle ログインとの連携なし

#### Page Contexts

コメントが表示される場所を制御します:

- **Course pages** - コースのメインページへのコメント
- **Module/activity pages** - 個々のアクティビティやリソースへのコメント
- **Both** - すべてのページタイプへのコメント

#### Commenting Style

コメントの体験を選択します。各モードのスクリーンショットは [Commenting Styles](#moodle-commenting-styles) を参照してください。

- **Comments** - ページコンテンツの下に表示される標準的なスレッド型コメントウィジェット
- **Collab Chat** - テキスト選択部分へのインラインディスカッション(プレゼンスインジケーター付き)
- **Both** - コメントとコラボチャットを同時に有効化

#### CDN URL

FastComments の CDN URL。デフォルトは `https://cdn.fastcomments.com` です。データが EU リージョンでホストされている場合は、EU 用の CDN URL に変更してください。