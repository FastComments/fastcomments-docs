---
Nastavite svoj tenant id enkrat v `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Nato dodajte tag kjerkoli želite pripomoček, v layoutu, prispevku ali strani:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

To je vse. Zamenjajte `demo` z vašim FastComments tenant id-jem (poiščete ga pod [Nastavitve > API/SSO](https://fastcomments.com/auth/my-account/api)).
---