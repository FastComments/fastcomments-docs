當使用者留言且未登入時，系統會要求他們提供電子郵件。

這會為該使用者建立一個「未驗證的工作階段」，我們會透過電子郵件要求他們驗證該工作階段。

對某些網站或應用程式而言，在留言或投票時不要求使用者提供電子郵件會比較理想。

啟用匿名留言功能後，電子郵件輸入欄位會變為選填。不過，我們也可以將其完全停用。首先，啟用匿名留言功能，然後停用電子郵件輸入欄位的選項就會出現。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments', '.disable-email-inputs']; selector = '.disable-email-inputs'; alt='在自訂介面中啟用匿名留言後顯示的選項，可完全隱藏電子郵件欄位'; title='停用電子郵件輸入欄位' app-screenshot-end]

啟用此選項後，我們所有的留言產品中都不會顯示電子郵件欄位。

請注意，在此設定下，除非使用者建立帳戶並登入 https://fastcomments.com，否則所有留言都會被標記為未驗證。

您可以考慮[停用未驗證標籤](/guide-customizations-and-configuration.html#disable-unverified-label)。