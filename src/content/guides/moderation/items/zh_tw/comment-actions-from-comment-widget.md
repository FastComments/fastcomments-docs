部分審核操作可以直接在留言串中執行，而無需前往 Comment Moderation 頁面。

當你登入後，點選留言右上角的編輯按鈕。身為版主，你應該會看到以下選項：

- **Pin** 該留言
- **Delete** 該留言
- **Delete** 該留言 + **Ban the user**（永久或隱形，詳情稍後）
- **Edit** 該留言
- **Lock** 或 **Unlock** 該留言（詳情見下方）
- 將該留言標記為 **Approved**（顯示）或 **Not Approved**（隱藏）
- 將該留言標記為 **Spam** 或 **Not Spam**

### Locking a Comment

鎖定個別留言會阻止任何對該留言的新增回覆，同時在解除鎖定之前，也會阻止該留言被編輯或刪除。這適用於所有人，包括管理員與版主。如果你需要編輯或移除鎖定的留言，請先解除鎖定、完成修改，然後視需要重新鎖定。

鎖定的留言在右上角會顯示一個小鎖圖示，讓讀者一目了然該討論串已關閉。在留言小工具與公開 API 中，鎖定留言的「編輯」與「刪除」選單項目會被隱藏（public API 中，當對鎖定的留言呼叫 `PATCH` 與 `DELETE` 時會回傳 `code: 'locked'`）。

有兩種刻意的例外會繞過鎖定，因為其它情況會留下孤立資料：當使用者刪除整個帳號時（其留言會無論鎖定狀態如何一併清除），以及當版主在封鎖使用者時勾選「delete all comments from this user」選項（清掃會清除鎖定）。

### Closing Comment Threads

版主與管理員可在留言區上方的三點選單中選擇 `Close Thread` 來鎖定或關閉留言串（需已登入）。他們也可以在任何時間選擇 `Re-Open Thread` 來重新開放留言功能。

關閉留言串會阻止新增留言，但仍允許投票，且使用者仍可刪除自己的留言（如有需要）。

關閉與重新開放留言串會即時影響所有正在檢視該串的使用者。

你也可以透過為該頁面建立自訂規則，將串標記為唯讀（read-only），這會同時移除投票與刪除選項。

### Updated Live

所有這些操作會立即更新其他使用者的留言串，無需重新載入頁面。然而，像隱藏留言或標記為垃圾訊息等版主操作，並不會從**版主的**畫面中移除該留言，以便在需要時能快速復原該操作。為了表示該留言已被隱藏，會以不同於其他留言的方式凸顯顯示（凸顯的顏色取決於移除的原因）。

For example, given users `A (commenter)`, `B (Moderator 1)`, and `C (Moderator 2)`.

...and the following scenario:

1. `User B (Moderator 1)` hides a comment.
2. For `User A (commenter)` that comment is immediately hidden.
3. For `User C (Moderator 2)` that comment is immediately hidden.
4. For the user that made the change, `User B (Moderator 1)`, the comment remains on their screen, but is highlighted as removed. They have the option to undo their action, in which case the other users will see the update, live, again.