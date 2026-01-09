[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

預設情況下，頂層留言的回覆會顯示。

可以設定成使用者需點選「顯示回覆」在頂層留言上才能查看子留言。

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = '將回覆摺疊至頂層留言'; code-example-end]

這可在小工具自訂頁面上無需撰寫程式碼地自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='摺疊回覆' app-screenshot-end]

此設定不會影響最初載入的頂層留言數量。如果您有一則頂層留言和 29 則子留言，啟用此設定後您會：

- 看見頂層留言。
- 在該留言下方看到「顯示回覆 (29)」。

如果您希望在使用此選項的同時顯示所有頂層留言，請將 [起始頁面設為 -1](#starting-page)。