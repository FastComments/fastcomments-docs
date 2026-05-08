[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 不會在頁面上顯示使用者名單。

您可以在評論元件旁顯示目前正在檢視該頁面的人員清單。該清單會在使用者加入或離開時即時更新，並顯示他們的名稱、大頭貼以及線上指示。

有三種版面配置選項：

- `1` - 上方：在評論上方呈現一列重疊的大頭貼。
- `2` - 左側：在元件左側顯示帶有名稱和線上點的側邊欄。
- `3` - 右側：在元件右側呈現相同的側邊欄。

設定 **usersListLocation** 標誌以啟用此功能：

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

預設情況下，該清單僅顯示目前在線的使用者。若要同時包含過去在此頁面留言但目前未在檢視的人員，請將 **usersListIncludeOffline** 設為 true：

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

過去的留言者呈現時不會顯示綠色的線上點，這樣可以清楚辨識目前哪些人是在線的。

具有私人檔案的使用者會顯示通用大頭貼和「私人檔案」標籤，這樣在不透露身分的情況下仍能保持計數的準確性。

這也可以在不使用程式碼的情況下設定。在元件自訂頁面中，請查看「使用者清單位置」選項。當該位置設定為非「關閉」時，下面會出現一個「包含過去的留言者」的核取方塊。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---