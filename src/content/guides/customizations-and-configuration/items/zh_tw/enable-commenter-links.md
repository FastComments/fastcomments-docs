[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 只會向使用者詢問他們的評論、使用者名稱以及電子郵件。

然而，在某些情況下，您可能希望使用者留下他們自己的部落格或網站連結。

我們可以透過將 **enableCommenterLinks** 旗標設為 true，來啟用顯示額外的輸入欄位，以留下使用者的網站 URL：

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = '啟用評論者連結'; code-example-end]

當提供該 URL 後，使用者的帳號將會被更新，且其過去與未來所有評論中的使用者名稱都會連結至此 URL。

這可以在小工具自訂頁面上，無需撰寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='已勾選評論者連結核取方塊以在評論表單中新增網站 URL 欄位的小工具自訂頁面'; title='啟用評論者連結' app-screenshot-end]