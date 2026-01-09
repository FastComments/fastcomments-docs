一個 `HashTag` 物件代表使用者可以留下的標籤。HashTag 可用於連結到外部的內容或將相關評論串連起來。

`HashTag` 物件的結構如下：

[inline-code-attrs-start title = 'HashTag 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** 應以 "#" 或其他想要的字元開頭。 **/
    tag: string
    /** 可選的 URL，hashtag 可指向此 URL。與其透過 hashtag 篩選評論，UI 在點擊時會導向此處。 **/
    url?: string
    /** 唯讀 **/
    createdAt: string
}
[inline-code-end]

注意：

- 在某些 API 端點中，你會看到 hashtag 被用在 URL 中。請記得對值進行 URI 編碼。例如，`#` 應該表示為 `%23`。
- 其中某些欄位被標示為 `READONLY` - 這些欄位由 API 回傳，但無法設定。