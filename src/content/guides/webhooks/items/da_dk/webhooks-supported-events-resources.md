---
FastComments understøtter kun webhooks for Comment‑ressourcen.

Vi understøtter webhooks for oprettelse, fjernelse og opdatering af kommentarer.

Hver af disse betragtes som separate hændelser i vores system og har derfor forskellige semantikker
og strukturer for webhook‑hændelserne.

Et vilkårligt antal slutpunkter kan abonnere på den samme hændelse: én webhook pr. domæne kan konfigureres i
dashboardet, og yderligere abonnementer kan oprettes via API’en (se Administrering af webhooks via API’en).

---