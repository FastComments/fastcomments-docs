[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會同時渲染評論輸入框和評論串。為了節省垂直空間，
它也會在未與小工具互動之前隱藏任何其他必填欄位。

然而，評論小工具可以隱藏在按鈕後面，例如：

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

按鈕會根據評論目前是否顯示而使用不同的翻譯文字。如果評論被隱藏，會使用 `translations.SHOW_COMMENTS_BUTTON_TEXT`。如果
評論已顯示，會使用 `translations.HIDE_COMMENTS_BUTTON_TEXT`。翻譯字串可以包含文字 `[count]`，該文字將
被替換為當地化的計數。

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

這是用來取代 `hideCommentsUnderCountTextFormat` 設定。

計數會隨評論串即時更新。如果沒有評論則不會顯示該按鈕。

可透過建立自定義規則並啟用「點擊以顯示評論」來在不撰寫程式碼的情況下啟用此功能：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]