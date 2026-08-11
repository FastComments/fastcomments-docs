---
[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會啟用即時評論功能。

這表示評論串的每位觀看者都應該看到相同的內容。

例如，若新增一則評論，該評論應該會顯示。若評論被編輯或刪除，
則所有觀看此串的使用者都會看到相同的編輯或刪除。投票以及所有審核操作亦同。

然而，我們可以停用此功能：

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

也可以不使用程式碼完成。在小工具自訂頁面，請參閱「停用即時評論」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='小工具自訂頁面的「停用即時評論」區段，關閉即時串更新'; title='停用即時評論' app-screenshot-end]

---