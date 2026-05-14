本指南說明在網站管理員已註冊該工具並設定為在活動選擇器中顯示後，如何將 FastComments 新增到 Moodle 4.x 課程。如果尚未註冊 FastComments，請先參閱 Moodle 註冊指南。

#### 開啟課程編輯模式

1. 使用課程的 Editing Teacher（或更高權限）身份登入 Moodle。
2. 開啟該課程。
3. 在課程標頭右上角使用開關切換 **Edit mode** 為開啟。

Moodle 4.x 用全螢幕的活動選擇器取代了 3.x 的舊式「Add an activity or resource」下拉選單。Moodle 4.5 在選擇器頂部新增了一列加星/我的最愛，將 FastComments 釘選一次之後，在後續章節會更容易存取。

#### 新增 FastComments 活動

1. 捲動到討論所屬的課程章節（主題或週次）。
2. 在該章節底部點選 **Add an activity or resource**。
3. 在選擇器對話框中，選擇 **FastComments**。如果看不到它，請跳到下面的注意事項章節。

活動設定表單會開啟。重要的欄位有：

- **Activity name**（必填）。顯示於課程頁面與成績冊。例如： `Week 3 Discussion`。
- **Activity description**。選用的簡介文字，會顯示在評論串上方。
- **Show description on course page**。如果想在不點入活動的情況下看到描述，請勾選此項。
- **Preconfigured tool**。設為 `FastComments`（從選擇器啟動時會自動選定）。請勿變更。
- **Launch container**。設為 **New window**。請參閱注意事項章節，說明為何選擇「Same window」在某些 Moodle 部署中會失敗。
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**。保持空白。網站層級的動態註冊會處理這些設定。

捲動到頁面底部並點選 **Save and return to course**（或 **Save and display** 以立即開啟該活動）。

該活動會以 FastComments 圖示顯示為章節中的一列。學生點選該列即可開啟評論串。

#### 在編輯器內嵌入 FastComments

若要在 Page、Book 章節、Lesson 或任何使用 Atto 或 TinyMCE 編輯器的資源內新增討論串：

1. 以編輯模式開啟資源。
2. 將游標放在希望顯示討論串的位置。
3. 在編輯器工具列中，點選 **LTI** / **External tool** 按鈕。在 Atto 中標為「Insert LTI Advantage content」。在 TinyMCE（Moodle 4.3+ 的預設）中，位於 **More** 功能表下的 **External tools**。
4. 從工具清單中選擇 **FastComments**。
5. FastComments 會開啟深度連結挑選器。確認討論串標題後點選 **Embed**。
6. 編輯器會插入 LTI 佔位區塊。儲存資源。

每個嵌入的實例都是一個以深度連結內容項目 ID 為鍵的獨立討論串，所以在同一 Page 中嵌入三個 FastComments 會得到三個獨立的討論串。

#### 存取限制與群組設定

FastComments 活動適用一般 Moodle 的活動設定：

- **Common module settings** > **Group mode**。將此設定為 **Separate groups** 或 **Visible groups** 並不會自動把 FastComments 拆成每群組各自的討論串。Moodle 的群組模式僅會過濾成績冊與成員列表。若要為每個群組運行獨立討論串，請為每個群組新增一個 FastComments 活動，並使用 **Restrict access** 來限定範圍。
- **Restrict access** > **Add restriction**。支援標準的 Moodle 條件：**Date**, **Grade**, **Group**, **Grouping**, **User profile**, 以及巢狀的限制組合。使用 **Group** 可將 FastComments 活動鎖定給單一群組。
- **Activity completion**。若想要完成度追蹤，請設定為 **Students must view this activity to complete it**。FastComments 目前在啟動之外不會回報其他完成事件給 Moodle。

#### 角色對應

FastComments 會讀取 Moodle 在每次啟動時傳送的 LTI `roles` 欄位，並做以下對應：

- Moodle **Manager** or **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** or **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> 僅可閱讀

Admins 可以刪除任何評論、封鎖使用者與編輯討論串設定。Moderators 可以在他們啟動進入的討論串內刪除與核准評論。自訂的 Moodle 角色會繼承其複製來源的 archetype 對應。

#### 學生會看到的內容

學生點選 FastComments 活動（或捲動到 Page 或 Book 內嵌的區塊）。Moodle 透過 LTI 啟動將他們的身份傳送給 FastComments：

- 無需登入畫面。FastComments 會使用 Moodle 帳號為他們登入。
- 他們的顯示名稱、電子郵件與頭像由 Moodle 提供。
- 討論串的範圍為 (Moodle site, course, resource link ID)，因此同一活動若複製到另一門課程會得到新的討論串。
- 分層回覆、投票與通知的功能與獨立的 FastComments 討論串相同。

#### Moodle 注意事項

**在活動選擇器中看不到 FastComments。** 可能是網站管理員已註冊該工具，但未將 **Tool configuration usage** 設為 **Show in activity chooser and as a preconfigured tool**。請到 **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > FastComments 欄位的齒輪圖示去修正。

**若設定為「Same window」啟動會失敗或顯示空白框架。** Moodle 的 session cookies 預設使用 `SameSite=Lax`，某些瀏覽器在 LTI 1.3 用於從 FastComments 返回的跨站 POST 上會移除這些 cookie。請在活動中將 **Launch container** 設為 **New window**。這對嵌入在 Page 或 Book 內的 FastComments 為硬性需求，因為編輯器內嵌的啟動流程會始終跳出新視窗。

**`iss` 欄位是 Moodle 網站 URL，而不是租戶 ID。** FastComments 使用 Moodle 網站 URL（`wwwroot` 設定值）作為 LTI issuer。如果你的 Moodle 實例搬到新網域或更改了 `wwwroot`，既有的 FastComments 討論串會維持綁定至舊的 issuer，將不會與新啟動相符。必要時請針對新 URL 重新註冊該工具，並透過 FastComments 管理介面遷移討論串。

**活動備份與還原。** 備份課程並還原到新課程會產生新的資源連結 ID，因此還原後的 FastComments 活動會從空白討論串開始。原始課程會保留原本的討論串。這是預期行為，非錯誤。

**Moodle 4.5 的 TinyMCE 預設。** Moodle 4.5 對新安裝而言預設使用 TinyMCE 編輯器。External tool 按鈕位於 **More**（`...`）功能表下，而非主要工具列。從 4.1 升級的舊站點會保留 Atto，除非管理員變更了預設設定。