---
在 `_config.yml` 中設定你的租戶 ID（只需一次）：

```yaml
fastcomments:
  tenant_id: demo
```

然後在想要放置元件的位置加入標籤，可以是佈局、文章或頁面：

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

就這樣。將 `demo` 替換成你的 FastComments 租戶 ID（可在
[設定 > API/SSO](https://fastcomments.com/auth/my-account/api) 找到）。
---