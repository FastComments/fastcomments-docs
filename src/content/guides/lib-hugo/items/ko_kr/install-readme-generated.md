Hugo 테마 컴포넌트를 추가하는 표준 방법 두 가지 중 하나를 선택하세요.

### 옵션 A: Hugo 모듈 (권장)

사이트 루트에서:

```bash
hugo mod init github.com/you/your-site   # only if your site is not already a module
hugo mod get github.com/FastComments/fastcomments-hugo
```

그런 다음 `hugo.toml`에 import 항목을 추가하세요:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### 옵션 B: 테마 컴포넌트 (Git 서브모듈)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

그런 다음 `hugo.toml`에서 참조하세요. 자신의 테마와 함께 나열하세요; 뒤에 있는 항목이 우선하므로 자신의 테마를 먼저 두세요:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```