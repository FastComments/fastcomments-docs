選擇以下兩種標準方式之一來新增 Hugo 主題元件。

### 選項 A：Hugo 模組（建議）

在你的網站根目錄：

```bash
hugo mod init github.com/you/your-site   # 只有當你的網站尚未是模組時才執行
hugo mod get github.com/FastComments/fastcomments-hugo
```

接著將匯入加入你的 `hugo.toml`：

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### 選項 B：主題元件（Git 子模組）

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

然後從你的 `hugo.toml` 參考它。與你自己的主題一起列出；後出現的項目會優先生效，所以保持你的主題在前面：

```toml
theme = ["your-theme", "fastcomments-hugo"]
```