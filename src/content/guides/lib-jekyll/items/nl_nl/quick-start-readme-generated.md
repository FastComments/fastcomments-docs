Stel je tenant-id één keer in in `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Voeg vervolgens een tag toe waar je de widget wilt hebben, in een layout, een bericht of een pagina:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Dat is alles. Vervang `demo` door je FastComments tenant-id (te vinden onder [Instellingen > API/SSO](https://fastcomments.com/auth/my-account/api)).