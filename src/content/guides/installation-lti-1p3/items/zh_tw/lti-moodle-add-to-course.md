本指南涵蓋在網站管理員已註冊該工具並設定為在活動選擇器中顯示後，如何將 FastComments 新增到 Moodle 4.x 課程。如果 FastComments 尚未註冊，請先參閱 Moodle 註冊指南。

#### 以編輯模式開啟課程

1. 以該課程的「編輯教師（或更高權限）」身分登入 Moodle。
2. 開啟課程。
3. 在課程標頭右上角使用切換開關將 **編輯模式** 開啟。

Moodle 4.x 用全螢幕的活動選擇器對話框取代了 3.x 的舊式「新增活動或資源」下拉式選單。Moodle 4.5 保留相同的選擇器，但在頂部新增了一列星號/我的最愛，因此將 FastComments 釘選後在之後的章節中會更容易存取。

#### 新增 FastComments 活動

1. 捲動到該討論所屬的課程章節（主題或週別）。
2. 點選該章節底部的 **新增活動或資源**。
3. 在選擇器對話框中，選擇 **FastComments**。如果看不到，請跳到下方的注意事項章節。

活動設定表單會開啟。重要的欄位：

- **Activity name** (required). 顯示在課程頁面和成績冊中。範例： `Week 3 Discussion`.
- **Activity description**。可選的介紹文字，呈現在留言串上方。
- **Show description on course page**。如果希望在不點入活動的情況下也能看到描述，請勾選此項。
- **Preconfigured tool**。設為 `FastComments`（從選擇器啟動時會自動選取）。請勿更改。
- **Launch container**。設為 **New window**。請參閱注意事項章節，了解為何在某些 Moodle 部署中選擇「Same window」會造成問題。
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**。保持空白。動態註冊已在網站層級處理這些設定。

捲動到底部並點選 **Save and return to course**（或 **Save and display** 以立即開啟該活動）。

該活動會以 FastComments 圖示的列出現在章節中。學生點選該列即可開啟留言串。

#### 在編輯器中內嵌 FastComments

若要在 Page、Book 章節、Lesson，或任何使用 Atto 或 TinyMCE 編輯器的資源內建立留言串：

1. 以編輯模式開啟該資源。
2. 將游標放在要顯示留言串的位置。
3. 在編輯器工具列中，點選 **LTI** / **External tool** 按鈕。在 Atto 中標示為「Insert LTI Advantage content」。在 TinyMCE（Moodle 4.3+ 的預設）中則位於 **More** 選單下的 **External tools**。
4. 從工具清單中選擇 **FastComments**。
5. FastComments 會開啟一個深度連結選擇器。確認留言串標題後點選 **Embed**。
6. 編輯器會插入一個 LTI 佔位區塊。儲存資源。

每個內嵌實例都是以深度連結的 content item ID 作為鍵的獨立留言串，因此在同一個 Page 中插入三個 FastComments 會有三個互不干涉的留言串。

#### 限制存取與群組設定

FastComments 活動適用標準 Moodle 的活動設定：

- **Common module settings** > **Group mode**。將此設為 **Separate groups** 或 **Visible groups** 不會自動把 FastComments 分割成每群組各自的留言串。Moodle 的群組模式只會過濾成績冊與成員清單。若要讓每個群組有各自的留言串，請為每個群組新增一個 FastComments 活動，並使用 **Restrict access** 來限制範圍。
- **Restrict access** > **Add restriction**。支援標準的 Moodle 條件：**Date**、**Grade**、**Group**、**Grouping**、**User profile**，以及巢狀的限制組合。使用 **Group** 可以將 FastComments 活動鎖定給單一群組。
- **Activity completion**。若想要完成度追蹤，請設為 **Students must view this activity to complete it**。FastComments 目前除了啟動時的 launch 外，並不會回報其他完成事件給 Moodle。

#### 角色對應

FastComments 會讀取 Moodle 在每次啟動時傳送的 LTI `roles` 宣告，並依下列方式對應：

- Moodle **Manager** 或 **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** 或 **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> 僅可閱讀

管理者可以刪除任何評論、封鎖使用者，以及編輯留言串設定。版主可以在其啟動進入的留言串內刪除及核准評論。自訂的 Moodle 角色會繼承其被複製來源原型的對應。

#### 學生會看到的內容

學生點選 FastComments 活動（或捲動到 Page 或 Book 內的內嵌區塊）。Moodle 透過 LTI 啟動將其身分傳送給 FastComments：

- 不會有登入畫面。FastComments 會使用 Moodle 帳號為他們登入。
- 他們的顯示名稱、電子郵件與大頭照會從 Moodle 取得。
- 留言串的範圍為「(Moodle site, course, resource link ID)」，因此相同活動若複製到另一門課程會得到新的留言串。
- 線性回覆、投票與通知等功能與獨立的 FastComments 留言串相同。

#### 鎖定公開存取（建議）

預設情況下，FastComments 的留言資料為公開可讀。任何能猜到留言串 URL 或 API 端點的人都可以查看其評論，即便是在 Moodle 之外。對於課程討論，你幾乎肯定會希望將檢視權限限制為僅限已註冊的學生。

開啟你的 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> 並建立一個規則，啟用 **Require SSO To View Comments**，然後將安全等級設為 **Secure SSO**，如此留言串就只能透過簽章的 LTI 啟動來載入。

完整的逐步說明（包含如何將規則限縮到單一網域或頁面）請參閱 [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments)。

#### Moodle 注意事項

**活動選擇器中看不到 FastComments。** 代表網站管理員已註冊該工具，但未將 **Tool configuration usage** 設為 **Show in activity chooser and as a preconfigured tool**。可在 **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > 在 FastComments 欄位點擊齒輪圖示來修正此設定。

**啟動失敗或在設定為「Same window」時顯示空白框架。** Moodle 的 session cookies 預設使用 `SameSite=Lax`，某些瀏覽器會在 LTI 1.3 用於從 FastComments 返回的跨站 POST 時移除這些 cookies。請在活動中將 **Launch container** 設為 **New window**。對於內嵌在 Page 或 Book 的 FastComments 這是強制需求，因為經由編輯器內嵌的啟動路徑總是會彈出新視窗。

**`iss` 宣告是 Moodle 網站 URL，而非租戶 ID。** FastComments 使用 Moodle 網站 URL（即 `wwwroot` 設定值）作為 LTI 的 issuer。如果你的 Moodle 實例搬到新網域或你變更了 `wwwroot`，既有的 FastComments 留言串仍會綁定到舊的 issuer，並不會與新的啟動相符。如有需要，請對新 URL 重新註冊該工具，並透過 FastComments 管理介面遷移留言串。

**活動備份與還原。** 備份課程並將其還原到新課程會產生新的 resource link ID，因此還原後的 FastComments 活動會從空的留言串開始。原課程會保留原始留言串。這是預期行為，非錯誤。

**Moodle 4.5 的 TinyMCE 預設。** Moodle 4.5 在新安裝時以 TinyMCE 當作預設編輯器。External tool 按鈕位置在 **More** (`...`) 選單下，而非主工具列。較舊從 4.1 升級的網站若未經管理員更改預設，會保留 Atto。