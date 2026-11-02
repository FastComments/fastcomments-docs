每個 WordPress 網站都需要自己的 FastComments 帳戶或租戶。WordPress 會在每個站點上分別編號文章，因此在一個站點的第 5 篇文章和另一個站點的第 5 篇文章，如果兩者都連接到同一個帳戶，將會共享同一個評論串。基於此原因，外掛的設定在帳戶已連接其他站點時會拒絕連接新站點。

[您可以在此查看您的活躍整合。](https://fastcomments.com/auth/my-account/manage-data/integrations)

要運行多個 WordPress 網站，請為每個網站建立一個[tenant](https://fastcomments.com/auth/my-account/tenants)，切換到該租戶，然後在外掛的設定頁面連接站點。租戶已包含在具白標功能的方案中。您也可以為每個站點建立單獨的 FastComments 帳戶，使用不同的電子郵件地址。

如果站點已遷移，例如從測試環境搬到正式環境，請在舊站點上停用外掛並從您的活躍整合中移除舊站點。然後連接新站點。已從舊站點同步的評論會保留在帳戶中，並會在新站點上具有相同 ID 的任何文章中顯示。

您可以在單一的 WordPress 安裝中使用相同的 FastComments 帳戶，該安裝跨多個域名且在這些域名上擁有相同的內容。