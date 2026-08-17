---
預設情況下，FastComments 會顯示使用者輸入的名稱，或是透過 SSO 傳遞給我們的名稱。

然而，有時可能需要以不同方式遮蔽或顯示使用者的名稱。例如，若使用者的名稱是 Allen Rex，您可能只想顯示「Allen R.」。

這可以在 Widget Customization UI 中，於名為 `Commenter Name Format` 的設定下，無需撰寫程式碼即可完成：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='評論者名稱格式下拉選單開啟，包含如「首字母大寫」、 「姓氏首字母」和「全部首字母」等選項'; title='變更名稱格式' app-screenshot-end]

可用的格式有：

- 首字母大寫（顯示範例使用者為 Example User）
- 姓氏首字母（顯示 Example User 為 Example U.）
- 全部首字母（顯示 Example User 為 E. U.）
- 顯示「Anonymous」

變更此設定的效果會立即生效。使用者仍會在評論區上方看到自己的完整使用者名稱，但其評論將顯示已修改的使用者名稱。

使用者名稱會在伺服器端被遮蔽，以保護使用者。
---