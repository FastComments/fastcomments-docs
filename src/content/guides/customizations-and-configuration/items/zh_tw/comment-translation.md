---
預設情況下，FastComments 會將小工具自身的介面（按鈕、標籤、錯誤訊息）翻譯成觀看者的語言，但評論會以原始撰寫的內容顯示。

「評論翻譯」功能會翻譯評論本身，讓使用多種語言撰寫的社群能夠閱讀每則評論的內容。

有兩種模式：

- **Automatic**：以非小工具語言撰寫的評論會顯示翻譯結果。評論下方會出現「Translated from Spanish」的文字，並附有 **Show original** 連結，可在翻譯與原文之間切換。
- **Manual**：每則評論都會有 **Translate** 連結。點擊後會將該評論翻譯成觀看者的語言，之後同樣提供 **Show original** 切換。已經是觀看者語言的評論則會顯示相應說明。

目標語言為小工具的語言：若同一自訂規則中已設定 [Locale / Language](#locale) 則使用該設定，否則使用訪客的瀏覽器語言。訪客不需要任何額外操作、帳號、擴充功能或軟體，且在嵌入式小工具與獨立評論頁面中皆以相同方式運作。

這項功能可於每個 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">customization rule</a> 中啟用，因而可套用於整個網站、單一網域或單一頁面。預設為關閉。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='在小工具自訂頁面上的「評論翻譯」下拉選單，包含 Off、Automatic 與 Manual 選項'; title='評論翻譯' app-screenshot-end]

翻譯是一項伺服器端設定。僅在小工具的 JavaScript 設定中傳入 `commentTranslation` 並不會啟用它；必須在自訂規則中啟用。

每則評論在每個目標語言下最多只會翻譯一次，且翻譯結果會與評論一起儲存，因此一則被千名西班牙語使用者閱讀的評論只會翻譯一次。若作者或審核者編輯了評論，已儲存的翻譯會被捨棄，且在下次檢視時重新翻譯。翻譯後的文字會經過與原始評論相同的清理與連結規則。過長的評論或無法偵測語言的評論將直接顯示原文。

使用量會以 LLM 代幣計費，費率與 [AI Agents](/guide-ai-agents.html#plans-and-eligibility) 相同，並會出現在您的帳單明細的相同位置。

---