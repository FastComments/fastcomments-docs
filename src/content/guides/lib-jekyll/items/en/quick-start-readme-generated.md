Set your tenant id once in `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Then add a tag wherever you want the widget, in a layout, a post, or a page:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

That's it. Replace `demo` with your FastComments tenant id (find it under
[Settings > API/SSO](https://fastcomments.com/auth/my-account/api)).