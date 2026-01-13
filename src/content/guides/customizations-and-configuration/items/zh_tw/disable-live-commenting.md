[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會啟用即時評論。

這表示評論串的每位檢視者都應該看到相同的內容。

例如，如果新增一則評論，該評論應該會顯示。如果評論被編輯或刪除，
那麼那些評論也會在該串的所有檢視者端被編輯或刪除。投票以及所有的管理操作亦同。

不過，我們可以停用此功能：

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

這也可以在不使用程式碼的情況下完成。在 widget 自訂頁面，請參閱「Disable Live Commenting」一節。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---