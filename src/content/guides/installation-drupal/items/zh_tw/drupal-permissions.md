此模組新增三個 Drupal 權限，可在 `People > Permissions` 下為每個角色分配。

- **Administer FastComments** - 可存取位於 `/admin/config/content/fastcomments` 的 FastComments 設定表單。
- **View FastComments** - 需要此權限才能看到留言小工具。沒有此權限時小工具不會渲染。
- **Toggle FastComments** - 允許使用者使用欄位小工具針對每個實體啟用或停用評論。

預設只有擁有 `administer site configuration` 權限的使用者可以變更 FastComments 設定。若您希望訪客能看到小工具，請將 `View FastComments` 權限授予匿名及已驗證的使用者。