All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## 必要

- **Tenant ID** - あなたの FastComments テナント IDです。これを [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）で確認できます。
- **API Secret** - Secure SSO、Webhook 検証、ページ同期に必要です。これを [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）で確認できます。

## API シークレットの保存

モジュールは、以下のソースのうち最初に値が設定されているものから API シークレットを読み取ります。

1. **Key モジュール** - [Key](https://www.drupal.org/project/key) をインストールし、*Authentication* キーを作成して、設定フォームの **API Secret key** で選択します。キーは環境変数またはウェブルート外のファイルでバックアップできます。
2. **settings.php** - `$config['fastcomments.settings']['api_secret'] = 'your-secret';` を `settings.php` または `settings.local.php` に追加します。シークレットはエクスポートされた設定に含まれません。
3. **設定フォーム** - **API Secret** フィールドに入力します。これにより値は `fastcomments.settings` に保存され、設定エクスポートに含まれます。Drupal のステータスレポートは、これがアクティブなソースであることを警告します。

フォームはどのソースがアクティブかを表示します。フィールドを空白のままにすると、保存されたシークレットが保持されます。シークレットを Key モジュールまたは `settings.php` に移動した後、**Remove the API Secret stored in configuration** にチェックを入れて削除してください。

## コメントスタイル

サイト上でユーザーがどのように会話するかに合わせてウィジェットを選択してください。

- **Live Comments** - リアルタイムのスレッドコメント。
- **Streaming Chat** - ライブチャットインターフェースで、イベントやライブストリームに最適です。
- **Collab Chat** - メインコンテンツ領域でテキスト選択に基づく注釈。訪問者はテキストをハイライトし、コンテキスト内でディスカッションを開始します。
- **Collab Chat + Comments** - 同じページ上でコラボチャットと標準コメントの両方を利用できます。

## SSO モード

- **None** - SSO なし。ユーザーはゲストとしてコメントするか、FastComments アカウントを作成します。
- **Simple** - サーバー側の検証なしで、Drupal のユーザー情報（名前、メール、アバター）を FastComments に渡します。
- **Secure** - HMAC-SHA256 を使用して Drupal ユーザーを FastComments と照合します。API シークレットが設定されている場合に推奨されます。

`Single Sign-On (SSO)` セクションをご参照ください。

## その他の設定

- **CDN URL** - デフォルトは `https://cdn.fastcomments.com` です。
- **Site URL** - デフォルトは `https://fastcomments.com` です。
- **Email notifications** - コンテンツに新しいコメントが投稿されたときに、コンテンツ作者へメールを送信します。

EU データレジデンシーについては、`EU Data Residency` セクションをご参照ください。