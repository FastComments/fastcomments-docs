---
預設情況下，FastComments 不會限制評論時使用的語言。 

可能需要限制社群使用的語言。

這可以在小工具自訂頁面上，無需編寫程式碼即可設定：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='在小工具自訂頁面上，用於限制評論可使用語言的允許語言選擇器'; title='允許的語言' app-screenshot-end]

系統會解析他們的評論並判斷其語言，然後與允許清單進行比對。

如果評論使用了未被允許的語言，系統會顯示本地化的錯誤訊息。 
---