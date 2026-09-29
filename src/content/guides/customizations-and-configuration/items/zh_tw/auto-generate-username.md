---
當使用者發表評論或投票，且未登入時，系統會要求他們提供電子郵件和使用者名稱。

對於某些網站而言，要求訪客自行想出唯一的使用者名稱是一個障礙，尤其在行動裝置上。FastComments 可以為每位新訪客產生一個中性的使用者名稱，並預先填入使用者名稱欄位，例如 `BraveOtter4172`。

訪客可以保持原樣，或自行更換為他們想要的名稱。

這項功能可在自訂 UI 中的 `Generate Usernames Automatically` 設定啟用：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='小工具自訂 UI 中的「自動產生使用者名稱」選項'; title='自動產生使用者名稱' app-screenshot-end]

#### 它的運作方式

- 每個產生的名稱都是唯一的。系統會檢查現有帳號，並為該訪客的瀏覽器會話保留該名稱，確保不會有兩位訪客被提供相同的名稱。
- 僅在訪客尚未擁有名稱時才會產生。已登入的使用者、SSO 使用者，以及已發表過評論的訪客會保留其現有名稱。
- 它在開啟或關閉[匿名評論](/guide-customizations-and-configuration.html#allow-anon)時皆可運作。若關閉匿名評論，訪客仍需輸入電子郵件，但不再需要思考使用者名稱。
- 再次造訪的訪客若輸入先前使用過的電子郵件，系統會將其對應到現有帳號，並保留該帳號的名稱。
- 如果同時設定了[預設使用者名稱](/guide-customizations-and-configuration.html#default-username)，產生的名稱將優先使用。

---