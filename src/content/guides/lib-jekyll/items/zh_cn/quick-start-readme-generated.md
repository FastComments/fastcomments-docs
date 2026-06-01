---
在 `_config.yml` 中设置一次您的租户 ID：

```yaml
fastcomments:
  tenant_id: demo
```

然后在布局、文章或页面的任意位置添加一个标签以放置小部件：

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

就这么简单。将 `demo` 替换为您的 FastComments 租户 ID（可在
[设置 > API/SSO](https://fastcomments.com/auth/my-account/api) 中找到）。
---