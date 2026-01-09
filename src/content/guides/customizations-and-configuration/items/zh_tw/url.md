[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

當傳送通知電子郵件，或在像審核頁面這類的使用者介面中呈現留言時，能夠從留言連結回其所在的頁面會很有幫助。

如果 URL ID 並不總是實際的識別碼，我們就必須在其他地方儲存 URL。這就是 "url" 屬性的用途，定義如下。

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

一個常見的使用情境是將留言串綁定到一個識別碼（例如文章），然後再連回特定頁面，例如：

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL 不會移除常見的行銷參數。預設情況下，會以當前頁面的 URL 作為與留言一起儲存的 URL。

---