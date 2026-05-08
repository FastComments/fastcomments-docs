[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 不會在頁面上顯示使用者清單。

您可以在留言元件旁顯示正在檢視該頁面的使用者清單。此清單會在使用者加入或離開時即時更新，並顯示他們的名稱、大頭貼與上線指示。

有三種佈局選項：

- `1` - Top: 在留言上方以水平重疊的頭像列顯示。
- `2` - Left: 在元件左側顯示含名稱與上線點的側邊欄。
- `3` - Right: 相同的側邊欄顯示在元件右側。

設定 **usersListLocation** 標記以啟用此功能：

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

預設情況下，清單僅會顯示目前上線的使用者。若要同時包含過去曾在此頁面留言的使用者（但目前未在檢視），請將 **usersListIncludeOffline** 設為 true：

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

過去的留言者不會顯示綠色的上線點，這樣可以清楚分辨目前在線的人員。

有私人檔案設定的使用者會顯示為通用大頭貼，並帶有「私人檔案」標籤，這樣可以在不揭露身分的情況下保持人數統計的準確性。

這也可以在不撰寫程式碼的情況下設定。在元件自訂頁面中，請查看「使用者清單位置」選項：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

當位置設定為非「關閉」時，下方會顯示「包含過去的留言者」的核取方塊：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]