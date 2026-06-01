---
Hugo テーマコンポーネントを追加するための標準的な2つの方法のうち、いずれかを選んでください。

### オプション A: Hugo モジュール（推奨）

サイトのルートで:

```bash
hugo mod init github.com/you/your-site   # サイトがまだモジュールでない場合のみ
hugo mod get github.com/FastComments/fastcomments-hugo
```

次に、`hugo.toml` にインポートを追加します:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### オプション B: テーマコンポーネント（Git サブモジュール）

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

次に、`hugo.toml` から参照します。自分のテーマと並べてリストしてください。後に書かれたエントリが優先されるので、あなたのテーマを先に置いてください:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```
---