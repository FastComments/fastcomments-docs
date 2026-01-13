[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 只會要求使用者輸入評論、使用者名稱和電子郵件。

不過在某些情況下，你可能想讓使用者留下指向他們自己的部落格或網站的連結。

我們可以透過將 **enableCommenterLinks** 標記設為 true 來啟用顯示額外的輸入欄位，讓使用者留下網站 URL：

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

當提供該 URL 時，使用者的帳戶會被更新，過去與未來的所有評論中的使用者名稱都會連結到這個 URL。

也可以在小工具（widget）自訂頁面上，在不寫程式碼的情況下進行設定：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---