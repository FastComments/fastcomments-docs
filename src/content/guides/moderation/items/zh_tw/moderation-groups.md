---
管理員可以被分配到群組中，以管理不同頁面或內容類別的評論。

當一位管理員屬於一個或多個群組時，他們在審核評論頁面中只會看到來自那些群組的評論。

例如，假設我們經營一個依分類顯示影片的網站。我們可能希望為貓、狗和鸚鵡影片分配不同的管理員，因此 [新增這些群組](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)。

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='The Moderation Groups Page' app-screenshot-end]

當我們新增一位管理員時，現在可以選擇一個或多個該管理員將屬於的群組：

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='Adding A Moderator and Selecting a Group' app-screenshot-end]

最後，評論需要被綁定到一個或多個群組，才能讓正確的管理員看到它們。

這可以透過 [新增一些群組](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) 然後在評論小工具中指定對應的 `Moderation Group` ids 來設定，
[如此處說明](/guide-customizations-and-configuration.html#moderation-group-ids).

---