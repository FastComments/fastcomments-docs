[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

使用 FastComments 時，評論小工具中的所有文字皆可自訂。

您可以覆寫單一文字，例如送出按鈕，或是整個評論小工具中的所有文字。

預設情況下，評論小工具中的文字會根據使用者的語系進行翻譯。然而，如果我們確信使用者群使用相同的地區/語言，我們可以覆寫文字，例如：

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

所有可自訂的翻譯可在 <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">此處</a> 的「進階選項」分頁中找到。

不過，有更簡單的方法，可透過小工具自訂 UI。在那裡，我們只要找到 EN_US 語系下評論小工具顯示的文字，並指定替換文字即可。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='自訂文字面板，從下拉選單中選取小工具字串並提供替換文字欄位'; title='自訂文字' app-screenshot-end]

所有翻譯覆寫目前會影響所有語系。

---