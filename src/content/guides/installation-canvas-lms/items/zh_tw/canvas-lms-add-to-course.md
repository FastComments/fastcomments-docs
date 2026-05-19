#### 課程中的留言顯示方式

一旦啟用 LTI 整合並安裝 External App，FastComments 會根據您設定的位置自動運作：

#### Assignment View

如果啟用 **Assignment View** 這個放置位置，留言會自動顯示在課程中每個作業的下方。學生與教師在檢視作業時會看到串狀留言區 — 每個作業不需額外設定。

每個作業都有獨立的留言討論串。

#### Rich Content Editor Button

如果啟用 **Editor Button** 這個放置位置，教師可以在任何使用 Rich Content Editor 的內容中嵌入 FastComments：

1. 編輯 **Page**, **Quiz**, 或 **Announcement**。
2. 在 Rich Content Editor 的工具列中，按一下 **FastComments** 按鈕。
3. FastComments 會自動嵌入到內容中。
4. 儲存頁面。

當學生檢視該頁面時，嵌入的 FastComments 小工具會載入該頁面專屬的留言討論串。

#### Automatic SSO

在這兩種放置位置中，學生會自動以其 Canvas 帳號登入。姓名、電子郵件與大頭照會透過 LTI 啟動同步，無需另外登入。

#### Lock Down Public Access (Recommended)

預設情況下，FastComments 的留言資料可公開閱讀。任何能猜到討論串的 URL 或 API endpoint 的人都可以查看其留言，即使在 Canvas 之外亦然。對於課程討論，您幾乎肯定會想限制只有已註冊的學生才能檢視。

開啟您的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">小工具自訂頁面</a>，建立一個啟用 **Require SSO To View Comments** 的規則，然後將安全等級設為 **Secure SSO**，這樣討論串就只能透過已簽署的 LTI 啟動來載入。

請參閱 [以單一登入保護留言討論串](/guide-customizations-and-configuration.html#sso-require-to-view-comments) 以取得完整操作流程，包括如何將規則範圍限定到單一網域或頁面。