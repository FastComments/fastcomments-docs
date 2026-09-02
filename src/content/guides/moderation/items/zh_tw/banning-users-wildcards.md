可以使用通配符封鎖使用特定電子郵件提供者的使用者。

例如，若您發現所有來自 **@bademail.com** 的評論都是垃圾訊息，您只需在新增被封鎖使用者的電子郵件輸入欄位中輸入 "*@bademail.com" 即可封鎖整個電子郵件提供者。

請注意電子郵件中的 @ 前面的 "*".

### Subdomains

域名封鎖同時也會涵蓋該域名的所有子域名。封鎖 `*@bademail.com` 也會封鎖
`someone@mail.bademail.com` 與 `someone@eu.mail.bademail.com`，因此無需為每個子域名分別新增封鎖。

如果您只想封鎖特定子域名，請改為輸入該子域名，例如 `*@mail.bademail.com`。此封鎖
不會影響 `someone@bademail.com`。

### Banning a Domain From a Comment

您不必自行輸入模式。當您在「審核評論」頁面從評論中封鎖使用者時，封鎖對話框
有一個 "封鎖所有 @domain 使用者" 核取方塊，會為該評論者的電子郵件域名建立相同的 `*@domain` 封鎖。

### Supported Patterns

唯一支援的通配符形式是以單一 `*` 取代整個名稱部分，後接 `@` 與域名。其他形式
在嘗試儲存時會被拒絕：

- `*@*.bademail.com` 並非必要，因為 `*@bademail.com` 已經涵蓋子域名。
- `name*@bademail.com` 與 `*bademail.com` 不受支援。