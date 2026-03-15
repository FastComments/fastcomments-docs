次の場所に移動：**管理 > 設定 > コンテンツ > FastComments** (`/admin/config/content/fastcomments`).

### 設定

- **Tenant ID** (required) - FastComments のテナント ID。これは [設定 > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）で確認できます。
- **API Secret** - Secure SSO、ウェブフック検証、およびページ同期に必要です。[設定 > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）で見つかります。
- **SSO Mode** - シングルサインオンの統合：
  - **None** - SSO は使用せず、ユーザーはゲストとしてコメントするか FastComments アカウントを作成します。
  - **Simple** - Drupal のユーザー情報（名前、メール、アバター）をサーバー側の検証なしで FastComments に渡します。
  - **Secure** - HMAC-SHA256 検証を使用して Drupal ユーザーを FastComments に安全に認証します（推奨）。
- **Commenting Style** - 表示するウィジェットの種類：
  - **Live Comments** - リアルタイムのスレッドコメント。
  - **Streaming Chat** - ライブチャットインターフェース。
  - **Collab Chat** - メインコンテンツ領域での共同テキスト選択注釈。
  - **Collab Chat + Comments** - Collab Chat と通常のコメントの両方。
- **CDN URL** - FastComments の CDN URL（デフォルト: `https://cdn.fastcomments.com`）。
- **Site URL** - FastComments のサイト URL（デフォルト: `https://fastcomments.com`）。
- **Email notifications** - 新しいコメントがコンテンツに投稿されたときに、コンテンツ作成者へメールを送信します。

### コンテンツタイプへのコメントの追加

**構造 > コンテンツタイプ > [type] > フィールドの管理** からコンテンツタイプに **FastComments** フィールドを追加します。フィールドにはステータスのトグルと、エンティティごとのオプションのカスタム識別子があります。

### EU データ居住

EU のデータ居住を有効にするには、次を更新してください：
- **CDN URL** を `https://cdn-eu.fastcomments.com` に変更
- **Site URL** を `https://eu.fastcomments.com` に変更