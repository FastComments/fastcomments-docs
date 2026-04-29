---
The **社群準則** 欄位在編輯表單上是一個可選的政策文字區塊，會在每次執行此代理時被包含進入使用者角色的上下文訊息。它被作為非受信任文字框起來（與平台套用於留言內容和其他使用者提供內容相同的框起方式），因此模型將其視為政策參考，而不是系統指示。這是記錄「在本網站上哪些行為被允許或不被允許」的規範性位置，以便代理能一致地套用它。

### 它與初始提示有何不同

- **初始提示** - 代理的角色與決策風格。「你是版主。比較偏好警告而不是禁言。」
- **社群準則** - 你社群的規則，以政策語言撰寫。「不得有人身攻擊。帳號未滿 24 小時不得張貼推廣連結。若討論串情緒激動，離題留言可能會被移除。」

兩者都會進入同一個上下文視窗，但它們進入不同層級 — 初始提示屬於系統角色的一部分，準則文件則是夾在使用者角色上下文訊息內的被框起文字。這個分離使得當你只想更新其中一項而不重新讀一次另一項時，編輯會更容易。

### 什麼是好的準則文件

一份簡短、具體、由人類撰寫的文件。與其用長篇敘述，不如用清單：

[inline-code-attrs-start title = '社群準則範例'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

代理會在每次執行時套用這些準則。如果你變更了準則，新內容會在下一次觸發時生效 — 過去的執行不會被追溯重新評估。

### 不要放在這裡的內容

- **輸出格式指示**（「以 HTML 回覆」、「使用表情符號」）。那些應放在 [初始提示](#personality-prompt)。
- **在地化文字。** 準則文件，和提示一樣，為相同理由應為 **English-only**（僅英文）。機器翻譯可能會悄悄改變代理的行為。如果你的政策依地區而異，請全部用英文寫在同一份文件，並把文件結構化為「針對德語頁面：...」之類的格式。
- **外部政策的長篇引用。** 改述要點。冗長的上下文會在每次執行時增加代幣成本。
- **個人識別資訊或機密。** 此文字會在每次執行時被送到 LLM 提供者。

### 長度

此欄位上限為 **4000 個字元**（由表單與儲存路由同時強制）。每次執行的代幣成本與長度成正比，所以即便在上限內，通常幾百字就已足夠。如果你的社群政策冗長數頁，請把代理需要的部分摘要成針對此欄位的重點。

### 版本控制

準則文件沒有內建的版本歷史 — 代理會使用最新儲存的值。如果你想保留歷史記錄，請在每次重大修改前把文件複製到你自己的追蹤系統。[Refine Prompts](#refining-prompts) 流程可以記錄 *初始提示* 的變更，但不會為準則文件建立版本紀錄。

---