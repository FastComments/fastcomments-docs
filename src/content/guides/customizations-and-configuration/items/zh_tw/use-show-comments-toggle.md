[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會同時呈現評論輸入框和評論串。為了節省垂直空間，它還會隱藏其他任何必填欄位，直到使用者與小工具互動為止。

然而，評論小工具可以隱藏在按鈕後面，例如：

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='評論小工具被折疊在一個顯示評論計數的按鈕後，直到讀者點擊它'; title='點擊以顯示評論' app-screenshot-end]

按鈕會根據評論目前是顯示還是隱藏而使用不同的翻譯文字。若評論被隱藏，會使用 `translations.SHOW_COMMENTS_BUTTON_TEXT`。若評論已顯示，則使用 `translations.HIDE_COMMENTS_BUTTON_TEXT`。翻譯文字可以包含 `[count]`，此佔位符會被本地化的計數取代。

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

此設計用於取代 `hideCommentsUnderCountTextFormat` 設定。

計數會隨著評論串即時更新。若沒有評論，按鈕不會顯示。

可以透過建立自訂規則並啟用「點擊以顯示評論」來在不撰寫程式碼的情況下啟用此功能：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='在小工具自訂頁面中，已勾選「點擊以顯示評論」的核取方塊的自訂規則'; title='啟用點擊以顯示評論' app-screenshot-end]

---