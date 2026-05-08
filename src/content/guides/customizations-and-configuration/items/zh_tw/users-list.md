[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 不會在頁面上顯示使用者清單。

您可以在留言小工具旁呈現目前正在檢視該頁面的人員清單。當使用者加入或離開時，清單會即時更新，並顯示他們的名稱、頭像和線上指示器。

有三種佈局選項：

- `1` - Top: 一列重疊頭像的水平排列，顯示於留言上方。
- `2` - Left: 左側的側邊欄，顯示名稱和線上點。
- `3` - Right: 相同的側邊欄顯示於留言右側。

將 **usersListLocation** 標誌設為啟用此功能：

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

預設情況下，清單只顯示目前線上的使用者。若要也包含過去在該頁面留言但目前並未檢視該頁面的人，請將 **usersListIncludeOffline** 設為 true：

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

過去的留言者會以沒有綠色線上圓點的方式呈現，這樣可以清楚區分現在誰是在線上的。

設定為私人個人資料的使用者會顯示通用頭像和「Private Profile」標籤，這樣在不揭露身分的情況下仍能保持計數準確。

這也可以在無需撰寫程式碼的情況下設定。在小工具自訂頁面中，請參閱「Users List Location」選項。當位置設定為非關閉（Off）時，下方會出現一個「Include past commenters」的勾選框。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

超過 500 名即時使用者後，清單可能最多落後 30 秒。