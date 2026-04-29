代理的 **tools** 是它可以採取的動作。代理編輯表單有一個 **Allowed tool calls** 區段，您可以在那裡勾選此代理可使用的工具，還有一個 **Approvals** 區段，您可以勾選那些在生效前需要人工核准的動作。

任何工具都有三個等級：

- **Disallowed** - 代理無法看見或使用它。
- **Allowed, no approval** - 代理直接使用它。記錄在執行歷史中。
- **Allowed, with approval** - 代理的呼叫會被放入人類審核佇列，只有在人工核准後才會執行。

被拒絕的工具是靜默的：代理不能請求它們，且平台會直接拒絕。需核准的工具則始終會通過 [approvals inbox](#approval-workflow)。

### Audit trail on every action

代理採取的每個動作都會記錄下簡短的理由（1–2 句說明原因）和一個信心分數（0.0–1.0）。兩者都會出現在 [Run Detail View](#run-detail-view) 與每一則 [approval](#approval-workflow) 上。搜尋記憶是唯一直讀取的例外：它不會被記錄為一個動作，且無論允許清單如何都始終可用。

### Tool reference

#### Posting comments

允許代理以自身身分發表留言。留言會以代理的顯示名稱公開顯示。此工具常由迎賓與摘要代理使用。可還原—任何版主都能移除不當留言。通常在不需核准的情況下允許；如果您的社群要求每則面向公眾的訊息都由人工審核，請將它設定為需核准。

#### Editing a comment

允許代理重寫範圍內留言的文字。原始文字會保存在留言的稽核日誌中。保留給範圍狹窄的情境——例如處理使用者洩漏的 PII 或修改代理自己先前的回覆。不應用於改寫意見或軟化語氣。**強烈建議設定為需核准。** 欲瞭解完整頁面，請參見 [Edit comment](#tool-edit-comment)。

#### Voting on comments

允許代理對留言投贊成或反對票。投票會像其他投票一樣計入留言的票數。大多數社群傾向不讓機器人投票；在任何初始範本中均未啟用。如果您允許它，投票是可還原的。

#### Pin / unpin a comment

允許代理將留言釘選至頁面頂端或取消已釘選的留言。平台不會強制每串只能有一則釘選，因此釘選代理應被指示先取消先前的釘選留言。由 Top Comment Pinner 範本使用。可還原；通常不需核准。

#### Lock / unlock a comment

允許代理阻止在留言下方繼續回覆，或恢復回覆。被鎖定的留言仍會顯示。適用於熱門討論的冷卻期，可搭配延後解鎖使用。可還原但會對您的社群可見；在風險較高的社群上，建議設定為需核准。

#### Mark / unmark spam

允許代理將留言標記為垃圾訊息（對讀者隱藏並餵給垃圾分類器）或清除該標記。是任何審核代理的日常工具。可還原。在建立對代理的信任期間（前幾週）應強烈考慮設定為需核准。

#### Approve / un-approve a comment

允許代理將被保留的留言展示給讀者，或隱藏已顯示的留言。對於那些將新留言保留供版主審核的租戶最為有用。當取消核准已顯示的留言時風險很高——請考慮在任何地方設定為需核准。

#### Mark a comment reviewed

佇列狀態工具：將留言標記為「版主（或代理）已查看」。不會改變可見性。風險低；很少設定為需核准。

#### Award a badge

允許代理根據您租戶的徽章設定授予使用者徽章。版主可以還原。很少設定為需核准。代理必須知道徽章 ID，因此請在您的 [community guidelines](#community-guidelines) 或 [initial prompt](#personality-prompt) 中包含相關 ID。

#### Send email

允許代理從 `noreply@fastcomments.com` 發送純文字電子郵件到其選定的地址。請謹慎使用——電子郵件是摩擦最高的工具，錯誤的郵件難以撤回。強烈建議設定為需核准，並將核准郵件導向給代理最終會寄信之收件匣的擁有者。

#### Save / search agent memory

一對配套工具，可讀寫有關觸發該代理的使用者的共用備註池。記憶是跨所有租戶代理共享的，因此分流代理的備註會影響審核代理的決策。搜尋為唯讀且始終可用；儲存很少設定為需核准。完整設計請見 [Agent Memory System](#agent-memory-system)。

#### Warn a user

向使用者傳送私訊，針對特定留言發出警告，並以原子方式在代理記憶中記錄該警告。平台的升級政策以此工具為核心——先警告，若使用者再犯則封鎖。通常比 `ban_user` 少被設定為需核准，但在代理啟用的前幾週內仍應考慮設定為需核准。完整頁面請見 [Warn user](#tool-warn-user)。

#### Ban a user

代理可以呼叫的最具決定性工具。以固定期間封鎖使用者，選擇性地作為 shadow ban，選擇性地同時封鎖 IP，亦可選擇刪除該使用者的所有留言。兩個具破壞性的選項（IP、刪除全部）需在編輯表單上另行選擇加入才能使用。在 EU 區域，所有封鎖皆需人工核准（請參見 [EU DSA Article 17 Compliance](#eu-dsa-compliance)）。在任何地方都強烈建議設定為需核准。完整頁面請見 [Ban user](#tool-ban-user)。

### Ban-tool sub-options

Ban 工具暴露兩個具破壞性的選項 - delete-all-comments and ban-by-IP - 在您透過編輯表單的 **Ban options** 區段選擇加入之前，這兩個選項對模型完全隱藏。即便模型虛構了參數，平台也會拒絕您未選擇加入的值。請參見 [Ban user](#tool-ban-user)。