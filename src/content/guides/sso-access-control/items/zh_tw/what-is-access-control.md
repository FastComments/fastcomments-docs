With FastComments SSO Access Control，有時稱為 RBAC，使用者可以被限制只能存取特定的頁面或評論串。此外，
使用者只能 `@mention` 同群組的其他人。

## 詳細說明

我們可以將 `Users` 以及選擇性地將 `Pages` 放入群組。

當 `Users` 被放入群組，且在 Widget Settings 中啟用 `Limit Comments by SSO User Groups` 時，使用者
將只會看到來自相同群組使用者的評論，且只能 `@mention` 相同群組的使用者。

此外，`Pages` 也可以被放入群組，然後使用者只能存取他們有權限存取的頁面的評論。

我們稱之為 "User-Level" 群組與 "Page-Level" 群組。

那麼，哪一種適合您？

#### 使用 User-Level 群組如果...

- 您想要使用相同的 `urlId` 值（頁面 URL 或文章 ID），但依群組限制評論。
- 例如，您希望有「New User」和「Veteran User」群組，且他們在相同頁面上永遠不會看到彼此的評論。

#### 使用 Page-Level 群組如果...

- 您的群組有特定的頁面。
- 例如，屬於「Public Pages」群組的使用者不應該查看「Top Secret」的文章。