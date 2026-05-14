一旦 FastComments 在平台上註冊，講師會使用平台的標準外部工具流程將其新增到課程內容。本頁涵蓋 Sakai 23.x 與 Schoology Enterprise。

#### Sakai

**1. 將 FastComments 新增到網站**

網站維護者按每個網站啟用此工具：

1. 開啟網站並在左側導覽中點選 **Site Info**。
2. 點選 **Manage Tools**。
3. 捲動到 **External Tools** 清單並切換 **FastComments** 為啟用。
4. 點選 **Continue**，檢查工具清單，然後點選 **Finish**。

FastComments 現在會顯示為網站左側導覽項目。

**2. 重新排序左側導覽項目**

前往 **Site Info** > **Tool Order**。拖曳 **FastComments** 到想要的位置並點選 **Save**。您也可以從此畫面重新命名導覽標籤或對學生隱藏它。

**3. 在 Lessons 頁面內嵌入**

若要將 FastComments 直接放在 Lessons 頁面內，而不是作為獨立的左側工具：

1. 在網站中打開 **Lessons** 工具。
2. 點選 **Add Content** > **Add External Tool**。
3. 從清單中選擇 **FastComments**。
4. 如果在註冊期間 FastComments 宣告了 Deep Linking，Sakai 會打開工具的內容選擇器，讓您可以選取或標記該討論串。若未宣告 Deep Linking，Sakai 則會插入預設的啟動連結。
5. 儲存 Lessons 項目。

每個嵌入實例都會有自己的討論串，其範圍綁定到該資源連結。

**4. 學生存取的權限調整**

Sakai 透過 Realms 控制外部工具的啟動。要確認學生可以啟動 FastComments：

1. 以 Sakai 管理員身分登入並開啟 **Administration Workspace** > **Realms**。
2. 打開相關的 realm（例如 `!site.template.course` 或特定網站的 realm）。
3. 確認 `access` 角色已啟用 `lti.launch`，且 **external.tools** 群組中的角色權限已授予。
4. 儲存 realm。

若要進行網站層級的覆寫，維護者可從 **Site Info** > **Tool Order** 調整每個角色的工具可見性，透過隱藏或顯示 FastComments。

**5. 學生會看到什麼**

學生點選 FastComments 左側導覽項目（或捲動到嵌入的 Lessons 區塊）即可直接進入有串的留言檢視。SSO 為自動：Sakai 在 LTI 啟動時傳送使用者身份，FastComments 以他們的 Sakai 帳號幫他們登入。

角色對應：

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (Administration Workspace 的管理員) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai 注意事項**

- **Manage Tools 中看不到工具。** 如果 FastComments 未出現在 External Tools 清單，Sakai 管理員需要打開工具註冊表（**Administration Workspace** > **External Tools** > **FastComments**）並將 **Stealthed** 設為 `false`。Stealthed 的工具會從每站的 Manage Tools 選擇器中隱藏。
- **在共用分頁的瀏覽器中啟動失敗。** Sakai 的入口 CSRF 令牌綁定於瀏覽器工作階段。如果學生在不同分頁登入兩個 Sakai 網站或有過期的工作階段，啟動會回傳 403。解法：關閉其他 Sakai 分頁，登出並重新登入，然後重新啟動。若此情況在整個叢集發生，管理員也可以提升 `sakai.csrf.token.cache.ttl`。
- **嵌入 iframe。** 確認 `lti.frameheight` 在 `sakai.properties` 中夠大（600 或更高），以免留言串在 Lessons 頁面內被裁切。

#### Schoology

Schoology Enterprise 有兩種安裝情境。新增工具到課程前請先確認適用哪種情境。

**1. 兩種安裝情境**

- **(a) 企業層級安裝。** Schoology 系統管理員在組織層級安裝 FastComments 並將其指派到所有課程或特定課程範本。講師可省略安裝步驟，直接進入「新增教材」。
- **(b) 講師自行安裝。** 講師可從 **Course Options** > **External Tools** > **Install LTI Apps** 在單一課程中安裝此工具。自行安裝前提是系統管理員已在組織層級核准 FastComments 應用程式。

**2. 將 FastComments 新增為課程教材**

在課程內：

1. 開啟課程並前往 **Materials**。
2. 點選 **Add Materials** > **Add File/Link/External Tool**。
3. 選擇 **External Tool**。
4. 從已註冊的工具清單中選取 **FastComments**。
5. 設定 **Name**（這是學生在教材清單中看到的名稱）及可選的 **Description**。
6. 保持 **Enable Grading**（成績回傳）為關閉。FastComments 不會回報成績到 Schoology，啟用成績回傳會建立一個空的成績欄位。
7. 點選 **Submit**。

該教材現在會出現在課程教材清單中，點選時會開啟 FastComments 討論串。

**3. 透過富文字編輯器內嵌入**

如果系統管理員在註冊期間為 FastComments 啟用了 Deep Linking 放置，講師可以在任何富文字欄位內嵌入討論串（作業說明、頁面內容、討論提示）：

1. 在目標頁面打開富文字編輯器。
2. 在工具列點選 **External Tool**（拼圖圖示）。
3. 選擇 **FastComments**。
4. 在深度連結對話框中設定嵌入並點選 **Insert**。
5. 儲存頁面。

如果富文字編輯器中沒有 External Tool 按鈕，表示此租戶對該工具關閉了 Deep Linking。請參閱下方的注意事項。

**4. 可見性與分組指派**

Schoology 透過 Course Options 對每個分組控制工具可用性：

1. 在課程中，點選 **Course Options** > **External Tools**。
2. 對於每個已安裝的 LTI 應用程式，您可以控制它是否對課程中的所有分組或僅特定分組可見。
3. 要限制 FastComments 僅對某些分組可見，取消勾選不應看到該工具的分組。
4. 分組層級的存取也會決定哪些分組會看到 **Add Materials** > **External Tool** 的 FastComments 項目。

**5. 學生會看到什麼**

學生點選 FastComments 教材（或捲動到內嵌內容）即可進入串狀討論。SSO 會透過 Schoology 的 LTI 啟動在他們的 Schoology 帳號下自動完成。

角色對應：

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology 注意事項**

- **僅限企業版。** 個人與免費的 Schoology 帳號無法安裝 LTI 1.3 工具。若您的租戶為免費方案，Course Options 中不會有 **External Tools** 選項。請升級到 Schoology Enterprise 以使用 FastComments。
- **租戶預設關閉 Deep Linking。** 某些 Schoology 租戶在組織層級限制 Deep Linking 放置。若發生此情況，講師只會看到 **Add Materials** > **External Tool** 的流程，而不會在富文字編輯器看到 External Tool 按鈕。要啟用內嵌放置，系統管理員需前往 **System Settings** > **Integration** > **LTI 1.3** > **FastComments** 並啟用 **Content Item / Deep Linking** 放置，然後儲存。
- **分組指派的覆寫。** 如果 FastComments 是在企業層級指派，但講師在 **Add Materials** 看不到它，表示該課程的分組在組織層級指派中被排除。請要求系統管理員將該分組加入 FastComments 的應用程式指派。
- **教材名稱 vs. 討論串身分。** 在 Schoology 中重新命名教材不會移動討論串。討論串是以 LTI 資源連結 ID 為鍵，因此重新命名會保留相同討論串；刪除並重新建立教材會建立一個新的、空的討論串。