---
評論可以使用下列範例語法進行搜尋：

- 模糊字詞搜尋： `cats love`
- 精確片語比對： `I love cats.`
- 精確完整評論比對： `exact="I love cats."`
  - 僅符合其整段文字完全等於該值（區分大小寫）的評論，而非僅包含該片語的評論。
- 依頁面標題： `page:"Page Title"`
  - 支援自動完成。
- 依頁面 URL： `page:"https://example.com/some-page"`
  - 支援自動完成。
- 依網站/網域： `site:mysite.com` or `domain:othersite.com`
- 依使用者： `user:"Bob"`
  - 支援自動完成。

您可以透過分享管理頁面的頁面 URL，將搜尋結果與其他審核員或管理員共用。搜尋欄位
的值會在您按下「Go」後包含在瀏覽器的 URL 中。

---