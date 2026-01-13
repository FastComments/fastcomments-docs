[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

使用 FastComments，留言小工具中的所有文字皆可自訂。

您可以覆寫單一段文字，例如送出按鈕，或是覆寫整個留言小工具中的所有文字。

預設情況下，留言小工具中的文字會根據使用者的語系進行翻譯。不過，如果我們確定使用者族群使用相同的地區/語言，我們可以覆寫這些文字，例如：

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

所有可自訂的翻譯可以在 <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">這裡</a> 的「進階選項」分頁中找到。

不過，有更簡單的做法，可以透過小工具自訂化的使用者介面。在那裡，我們只要找到在 EN_US 區域設定中顯示於留言小工具的文字，然後指定一個替換字串即可。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

目前所有翻譯覆蓋都會影響所有語系。