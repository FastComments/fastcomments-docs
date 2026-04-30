**模板 ID：** `gaslight_detector`

Gaslight 偵測器會監控在討論進行中改寫歷史的留言編輯——那種作者在回覆發出之後改變早期留言的含義，導致後續回覆看起來不合脈絡或錯誤。當代理判斷編輯已超出界線時，它會還原原始文字並私訊作者說明。

這是一個較高風險的範本，因為它會修改使用者內容。在將其從 [dry-run](#dry-run-mode) 切換為正式啟用之前，請比只讀範本更長時間以 dry-run 運行，並在你信任模型在你的流量下的判斷之前，將 `edit_comment` 設為需經過 [approval](#approval-workflow)。

### 觸發條件

- **Comment edited** (`COMMENT_EDIT`) - 代理會比較新舊文字，並判斷該編輯是否扭曲已存在的回覆。

請參閱 [Trigger: Comment Edited](#trigger-comment-edit) 取得完整載荷，包括先前的留言文字與編輯時的回覆數。

### 允許的工具

- [`edit_comment`](#tool-edit-comment) - 當編輯被判定為 gaslighting 時，用來還原原始文字。
- [`warn_user`](#tool-warn-user) - 發出使用者下次訪問時會看到的軟性警告。
- [`send_dm`](#tools-overview) - 說明管道；使用者會收到說明其編輯被還原原因的直接訊息。

它不能封鎖使用者、標記為垃圾郵件、投票或發表新留言——功能刻意設計得很窄。

### 上線前建議的補充項目

- **將 `edit_comment` 設為需經過 [approval](#approval-workflow)。** 還原留言會對作者以及看過已編輯版本的任何人可見，因此誤判會很令人尷尬。在 dry-run 顯示代理判斷一致之前，請保持審核開啟。
- **在提示中明確說明在你網站上何者算作 gaslighting。** 預設提示刻意較簡短。給模型具體範例——「翻轉是/否的主張」、「刪除被回覆引用的數字」、「在回覆發出後加入敵意句子」——以及明確的非範例，如修正拼字錯誤、格式清理或加入來源。
- **在提示中使用觸發情境的回覆數。** 對於回覆數為零的留言的編輯無法扭曲對話；提示應告訴模型跳過那些情況。
- **在 [Context Options](#context-options) 中勾選「Include commenter's trust factor, account age, ban history, and recent comments」。** 當模型能看到長期良好紀錄的帳號時，它會減少激進判斷。
- **考慮在提示中加入短暫的編輯寬限期。** 許多在前 30–60 秒內的編輯是打字錯誤修正；指示模型忽略那些過快的編輯。

### 建議的 dry-run 期間

至少以真實流量在 [dry-run](#dry-run-mode) 運行兩週以上，然後再切換為啟用，並在該期間檢視每一個被標示的編輯。使用 [Test Runs (Replays)](#test-runs-replays) 在上線前將過去 30 天的編輯回放給代理測試。