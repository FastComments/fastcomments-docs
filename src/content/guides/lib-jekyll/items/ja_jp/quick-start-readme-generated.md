一度だけテナントIDを `_config.yml` に設定します：

```yaml
fastcomments:
  tenant_id: demo
```

次に、ウィジェットを表示したい場所（レイアウト、投稿、ページなど）にタグを追加します：

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

これで完了です。`demo` をあなたの FastComments テナントID に置き換えてください（確認は
[設定 > API/SSO](https://fastcomments.com/auth/my-account/api)）。