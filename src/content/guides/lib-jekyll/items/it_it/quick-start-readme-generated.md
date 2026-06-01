---
Imposta il tuo tenant id una sola volta in `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Poi aggiungi un tag dove vuoi il widget, in un layout, in un post, o in una pagina:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Ecco fatto. Sostituisci `demo` con il tuo tenant id di FastComments (lo trovi sotto
[Impostazioni > API/SSO](https://fastcomments.com/auth/my-account/api)).
---