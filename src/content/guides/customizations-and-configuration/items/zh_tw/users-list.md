[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 不會在頁面上顯示使用者列表。

您可以在評論小工具旁顯示目前正在檢視頁面的使用者列表。該列表會即時更新，隨著使用者加入或離開，並顯示他們的姓名、頭像以及線上指示器。

有三種版面配置選項：

- `1` - Top: 在評論上方呈現的水平排列、重疊的頭像列。
- `2` - Left: 在小工具左側呈現的側邊欄，顯示姓名與線上點。
- `3` - Right: 同樣的側邊欄顯示在小工具右側。

設定 **usersListLocation** 旗標以啟用此功能：

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = '在右側顯示使用者列表'; code-example-end]

預設情況下，列表僅顯示目前線上的使用者。若要同時包含過去在頁面上發表過評論（但目前未在檢視）的使用者，請將 **usersListIncludeOffline** 設為 true：

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = '包含過去的評論者'; code-example-end]

過去的評論者會以不帶綠色線上點的方式呈現，以便清楚顯示目前在場的使用者。

擁有私人檔案的使用者會顯示通用頭像與「私人檔案」標籤，確保人數統計正確且不洩露身分。

此設定也可透過介面完成。於小工具自訂頁面中，請參考「Users List Location」選項。當位置設定為非「關閉」時，會在其下方顯示「包含過去的評論者」勾選框。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='使用者列表位置設定為右側，且下方顯示「包含過去的評論者」勾選框'; title='使用者列表設定'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

過去 500 名即時使用者，列表最多延遲 30 秒。