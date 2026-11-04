可以將審核員分組，以審核不同的頁面或內容類別。

當審核員屬於一個或多個群組時，他們在「審核留言」頁面中只會看到這些群組的留言。

例如，假設我們經營一個按類別顯示影片的網站。我們可能希望為貓、狗和鸚鵡影片設置不同的審核員，因此[讓我們新增這些群組](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)。

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='審核群組列表，其中已為每個影片類別建立了貓、狗和鸚鵡群組'; title='審核群組頁面' app-screenshot-end]

當我們新增審核員時，現在可以選擇該審核員所屬的一個或多個群組：

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='新增審核員表單，其中的群組選擇器用於將審核員指派到一個或多個群組'; title='新增審核員並選擇群組' app-screenshot-end]

最後，留言需要與一個或多個群組關聯，以便正確的審核員能看到它們。

這可以透過[新增一些群組](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)，然後在留言小工具中指定對應的 `Moderation Group` id 來完成設定，
[如這裡的說明](/guide-customizations-and-configuration.html#moderation-group-ids)。