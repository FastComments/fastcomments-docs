---
預設情況下，FastComments 需要使用者提供電子郵件才能發表評論。雖然不必是有效的電子郵件，但在使用者點擊寄給他的連結之前，他的評論會顯示「未驗證的評論」標籤。

然而，我們可以移除電子郵件的必填要求。電子郵件輸入欄位仍會顯示，但不再是必填。

這可以透過小工具自訂 UI 進行設定：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.allow-anonymous-comments'; alt='在小工具自訂 UI 中的匿名評論選項，使電子郵件欄位變為可選'; title='啟用匿名評論' app-screenshot-end]

仍然需要使用者名稱。若也想省去此步驟，您可以
[設定預設使用者名稱](/guide-customizations-and-configuration.html#default-username)讓所有人共用，或讓 FastComments
[產生唯一的使用者名稱](/guide-customizations-and-configuration.html#auto-generate-username)為每位訪客。

---