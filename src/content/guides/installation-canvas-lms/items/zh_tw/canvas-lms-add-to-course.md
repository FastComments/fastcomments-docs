#### 留言在您的課程中如何顯示

一旦啟用 LTI 整合並安裝了 External App,FastComments 會根據您設定的 placements 自動運作:

#### 作業檢視

如果啟用了 **Assignment View** placement,留言會自動顯示在課程中每個作業的下方。學生和講師在檢視作業時會看到一個巢狀留言區 — 每個作業無需額外設定。

每個作業都有自己獨立的留言串。

#### Rich Content Editor 按鈕

如果啟用了 **Editor Button** placement,講師可以將 FastComments 嵌入到任何使用 Rich Content Editor 的內容中:

1. 編輯 **Page**、**Quiz** 或 **Announcement**。
2. 在 Rich Content Editor 工具列中,點擊 **FastComments** 按鈕。
3. FastComments 會自動嵌入到內容中。
4. 儲存頁面。

當學生檢視該頁面時,嵌入的 FastComments 小工具會載入該頁面專屬的留言串。

#### 自動 SSO

在兩種 placement 中,學生都會透過其 Canvas 帳號自動登入。姓名、電子郵件和頭像會透過 LTI launch 同步,無需另外登入。

#### 鎖定公開存取(建議)

預設情況下,FastComments 的留言資料是公開可讀的。任何能猜出留言串 URL 或 API endpoint 的人都可以檢視其留言,即使在 Canvas 之外也一樣。對於課程討論,您幾乎肯定會希望將檢視權限限制為僅限已註冊的學生。

開啟您的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> 並建立一條啟用 **Require SSO To View Comments** 的規則,然後將安全層級設為 **Secure SSO**,這樣留言串就只能透過已簽署的 LTI launch 載入。

完整操作說明請參閱 [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments),包括如何將規則範圍限制在單一網域或頁面。