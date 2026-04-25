すべての設定は `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`) にあります。

## 必須

- **Tenant ID** - あなたの FastComments Tenant ID。次の場所で確認できます: [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Secure SSO、webhook の検証、およびページ同期に必要です。次の場所で見つかります: [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## コメントスタイル

サイト上でユーザーにどのように交流してほしいかに合わせてウィジェットを選択してください。

- **Live Comments** - リアルタイムのスレッド形式コメント。
- **Streaming Chat** - ライブチャットインターフェース。イベントやライブ配信に適しています。
- **Collab Chat** - メインコンテンツ領域でのテキスト選択注釈。訪問者がテキストをハイライトして、その文脈で議論を開始できます。
- **Collab Chat + Comments** - 同じページで collab chat と標準のコメントの両方を表示します。

## SSO モード

- **None** - SSO を使用しません。ユーザーはゲストとしてコメントするか、FastComments アカウントを作成します。
- **Simple** - Drupal のユーザー情報 (name, email, avatar) をサーバー側で検証せずに FastComments に渡します。
- **Secure** - HMAC-SHA256 を使用して Drupal ユーザーを FastComments と照合します。API Secret を設定している場合に推奨されます。

詳細は `Single Sign-On (SSO)` セクションを参照してください。

## その他の設定

- **CDN URL** - デフォルトは `https://cdn.fastcomments.com` です。
- **Site URL** - デフォルトは `https://fastcomments.com` です。
- **Email notifications** - コンテンツに新しいコメントが投稿されたとき、コンテンツの作成者にメールを送信します。

EU のデータ居住については、`EU Data Residency` セクションを参照してください。