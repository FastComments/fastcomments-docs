---
Tenant id'nizi bir kez `_config.yml` içinde ayarlayın:

```yaml
fastcomments:
  tenant_id: demo
```

Ardından widget'ı istediğiniz herhangi bir yere, bir layout'a, bir gönderiye veya bir sayfaya bir etiket ekleyin:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

Hepsi bu. `demo`'yu FastComments tenant id'nizle değiştirin (bulmak için [Ayarlar > API/SSO](https://fastcomments.com/auth/my-account/api)).
---