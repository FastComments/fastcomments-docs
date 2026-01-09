預設情況下，FastComments 會顯示使用者輸入的名稱，或透過 SSO 傳給我們的名稱。

不過，可能希望以不同方式遮罩或顯示使用者名稱。例如，如果使用者的名字是 Allen Rex，您可能只想顯示 "Allen R."。

這可在 Widget 自訂 UI 中完成，無需撰寫程式碼，位於名為 `Commenter Name Format` 的設定下：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

The available formats are:

- Capitalize (display example user as Example User)
- Last Initial (display Example User as Example U.)
- All Initials (display Example User as E. U.)
- Show "Anonymous"

變更後會立即生效。使用者自己仍會在評論區上方看到完整的使用者名稱，但他們的評論將顯示
已被修改過的使用者名稱。

使用者名稱在伺服器端會被遮罩以保護使用者。