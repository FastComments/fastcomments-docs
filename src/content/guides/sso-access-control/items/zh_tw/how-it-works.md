FastComments 的存取控制透過將 `Pages` 和 `Users` 指派到所需的群組來運作。

群組只是字串識別碼，例如 `GREEN` 或 `abc-123`。

`Users` 與 `Pages` 並不限於單一群組。它們各自最多可被指派至 `100` 與 `1000` 個群組。 

#### 存取未授權的頁面

如果使用者嘗試存取其無權限的頁面，會看到如下的錯誤訊息：

<div class="screenshot white-bg">
    <div class="title">授權失敗範例</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="授權失敗範例" />
</div>

訊息文字可以自訂。

---