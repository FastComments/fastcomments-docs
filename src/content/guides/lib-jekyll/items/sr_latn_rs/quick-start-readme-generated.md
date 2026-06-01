Podesite svoj tenant id jednom u `_config.yml`:

```yaml
fastcomments:
  tenant_id: demo
```

Zatim dodajte tag gde god želite widget, u layoutu, postu ili stranici:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

To je to. Zamenite `demo` svojim FastComments tenant id-jem (pronađite ga pod
[Settings > API/SSO](https://fastcomments.com/auth/my-account/api)).