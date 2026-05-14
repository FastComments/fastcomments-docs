一旦 FastComments 在您的 LMS 中註冊後，教師可以像加入其他 LTI 外部工具一樣，將它加入課程。

#### D2L Brightspace

在課程的內容區：

1. 點選 **Add Existing Activities** > **External Learning Tools**。
2. 從清單中選擇 **FastComments**。
3. 該工具會以一個主題出現在內容區。打開它一次以初始化該資源的評論串。

#### Moodle

在課程中：

1. 開啟 **Edit mode**。
2. 在您想要放置評論的段落，點選 **Add an activity or resource**。
3. 在活動選擇器中選擇 **FastComments**。
4. 儲存。學生會在該段落中看到嵌入的評論串。

#### Blackboard Learn

在課程中：

1. 前往內容區 (Content Area)。
2. 點選 **Build Content** > **FastComments**（位於「Learning Tools」下）。
3. 設定名稱並提交。

#### Sakai

網站管理者透過 **Site Info** > **Manage Tools** > 捲動到 **External Tools** > 選取 **FastComments** > **Continue** 來新增工具。

#### How Threads Are Scoped

FastComments 會為每個 **(LMS instance, course, resource link)** 建立獨立的評論串。這表示：

- 即使使用相同的工具名稱，同一 LMS 中的兩個不同課程也會有各自獨立的評論串。
- 在同一課程中同一個 FastComments 工具若在兩個不同位置使用，會產生兩個評論串。
- 在同一個 FastComments 帳戶下的兩個不同 Brightspace 安裝會有不同的評論串 — LMS 主機名稱是評論串識別的一部分。

#### SSO

學生不會看到登入畫面。LMS 透過 LTI 啟動將他們的身分（姓名、電子郵件、頭像、角色）傳送給 FastComments，FastComments 會自動為他們登入。他們的留言會歸屬到其 LMS 帳戶。

具有 LMS 角色 **教師** 或 **系統管理員** 的使用者會自動對應為該評論串的 FastComments 管理員/管理角色。