[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

默認情況下，頂層評論的回覆會顯示。

可以將其設定為使用者必須點擊頂層評論上的「Show Replies」才能看到子回覆。

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

這可以在小工具自訂頁面上，無需編寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='在小工具自訂介面中的折疊回覆選項，將子評論隱藏在「Show Replies」連結後'; title='折疊回覆' app-screenshot-end]

此設定不會影響最初載入的頂層評論數量。若您只有一則頂層評論，且有 29 個子評論，啟用此設定後，您將會：

- 看到該頂層評論。
- 在此評論下看到「Show Replies (29)」。

若您希望在使用此選項的同時顯示所有頂層評論，請將 [起始頁面設為 -1](#starting-page) 設為 -1。