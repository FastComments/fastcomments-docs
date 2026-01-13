Per usare l'API per creare un tenant white-label, dobbiamo fare quanto segue:

1. Chiamare la [Tenants API](/guide-api.html#tenant-structure) per creare il tenant.
2. Chiamare la [TenantPackages API](/guide-api.html#tenant-package-structure) per creare uno o più pacchetti per il tenant.
3. Chiamare la [Tenants API](/guide-api.html#tenant-patch) per definire quale pacchetto è attivo sul tenant.
4. Chiamare la [TenantUsers API](/guide-api.html#tenant-user-structure) per aggiungere utenti amministratori al tenant.
5. Chiamare la [Moderators API](/guide-api.html#moderator-structure) per aggiungere e invitare moderatori al tenant.
6. Opzionalmente, [Configurare SSO](/guide-customizations-and-configuration.html#sso).

Tutto ciò può essere fatto durante il periodo di prova.