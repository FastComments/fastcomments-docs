Postavite svoj tenant id jednom u `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Zatim dodajte tag gdje god želite widget, u layout, objavu ili stranicu:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

To je to. Zamijenite `demo` sa vašim FastComments tenant id-om (pronađite ga pod [Postavke > API/SSO](https://fastcomments.com/auth/my-account/api)).