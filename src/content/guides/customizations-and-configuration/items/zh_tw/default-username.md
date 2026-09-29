---
當使用者發表評論或投票，且未登入時，系統會要求他們提供電子郵件和使用者名稱。

在匿名評論的情況下，有時希望設定預設使用者名稱以降低發表評論的阻力。這可以在自訂 UI 中完成。必須先啟用匿名評論。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments']; selector = '.default-username-label'; alt='在啟用匿名評論後，於自訂 UI 中出現的預設使用者名稱欄位'; title='設定預設使用者名稱' app-screenshot-end]

#### 分享預設使用者名稱

預設使用者名稱是一個共享的顯示名稱，而非身份識別。保留預設名稱並輸入電子郵件的訪客會擁有自己的帳號，且預設名稱會顯示為其公開名稱。任意數量的訪客都可以使用相同的預設名稱，因此「Anonymous」永不會被報告為已被使用。

訪客自行輸入的使用者名稱仍必須是唯一的。

如果您希望每位訪客都能取得獨特的名稱，而不必自行思考，請參閱[自動產生使用者名稱](/guide-customizations-and-configuration.html#auto-generate-username)。

---