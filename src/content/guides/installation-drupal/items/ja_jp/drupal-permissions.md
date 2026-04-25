このモジュールは、`People > Permissions` の下でロールごとに割り当てできる3つのDrupal権限を追加します。

- **Administer FastComments** - `/admin/config/content/fastcomments` にある FastComments 設定フォームへのアクセス。
- **View FastComments** - コメントウィジェットを表示するために必要です。この権限がないとウィジェットは表示されません。
- **Toggle FastComments** - フィールドウィジェットを使用して、エンティティごとにコメントを有効または無効にできるようにします。

---

デフォルトでは、`administer site configuration` 権限を持つユーザーのみが FastComments の設定を変更できます。訪問者にウィジェットを表示したい場合は、匿名および認証済みユーザーに `View FastComments` を付与してください。