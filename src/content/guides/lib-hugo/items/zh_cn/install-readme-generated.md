从以下两种标准方法中选择一种来添加 Hugo 主题组件。

### 选项 A：Hugo 模块（推荐）

在你的网站根目录下：

```bash
hugo mod init github.com/you/your-site   # 仅当你的网站尚未是一个模块时
hugo mod get github.com/FastComments/fastcomments-hugo
```

然后将导入添加到你的 `hugo.toml`：

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### 选项 B：主题组件（Git 子模块）

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

然后在你的 `hugo.toml` 中引用它。将它与你自己的主题一起列出；后出现的条目将生效，因此请将你的主题放在第一位：

```toml
theme = ["your-theme", "fastcomments-hugo"]
```