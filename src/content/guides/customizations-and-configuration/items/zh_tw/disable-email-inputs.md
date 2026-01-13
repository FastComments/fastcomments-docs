當使用者發表留言且尚未登入時，系統會要求他們提供電子郵件。

這會為該使用者建立一個「未驗證的工作階段」，並透過電子郵件要求他們驗證該工作階段。

對於某些網站或應用程式，當使用者評論或投票時不要求提供電子郵件可能比較合適。

啟用匿名留言會使電子郵件輸入欄位變為選填。然而，我們也可以將其完全停用。請先啟用
anonymous commenting，然後停用電子郵件輸入欄位的選項便會出現。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments', '.disable-email-inputs']; selector = '.disable-email-inputs'; title='Disable Email Inputs' app-screenshot-end]

啟用此設定後，電子郵件欄位將在我們所有的留言產品中完全不顯示。

請注意，使用此設定時，所有留言都會是未驗證狀態，除非使用者創建帳號並登入
https://fastcomments.com。

您可能想考慮[停用「未驗證」標籤](/guide-customizations-and-configuration.html#disable-unverified-label)。