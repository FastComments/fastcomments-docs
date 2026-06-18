---
한 번만 `hugo.toml`에 tenant ID를 설정하세요:

```toml
[params.fastcomments]
  tenantId = "demo"   # "demo"을 귀하의 FastComments tenant ID로 교체하세요
```

그런 다음 댓글 위젯을 테마에 연결하세요(자세한 내용은 [테마 통합](#theme-integration-readme-generated)을 참조), 또는 페이지의 Markdown에 숏코드를 삽입하세요:

```text
\{{< fastcomments >}}
```
---