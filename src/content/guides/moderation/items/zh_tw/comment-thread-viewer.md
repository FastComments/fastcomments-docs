---
當審核與檢視評論串時，能直接跳到特定串以取得審核時的上下文是很理想的。

這表示使用者的流程會從「評論審核」頁面開始，然後必須從單一評論跳到
包含該評論的頁面，等待該頁面載入，等待評論載入，然後再捲動到該評論。

然而，FastComments 提供了更快的方式。在「審核評論」頁面中，每則評論的右下角都有一個「檢視評論」按鈕。

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; alt='審核清單中的單一評論，右下角有「檢視評論」按鈕'; title='一則評論' app-screenshot-end]

如果此評論有回覆，按鈕文字會改為回覆數量，但點擊後執行的動作相同。

此按鈕會將您帶到 **Comment Thread Viewer**。

Comment Thread Viewer 是 FastComments 所託管的一個小型、快速載入的應用程式，會為該評論所在的頁面渲染評論串，並捲動至該評論。

這讓審核者能快速取得所需的上下文，而不必等待另一個頁面載入。

---