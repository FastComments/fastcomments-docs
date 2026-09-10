---
FastComments understøtter kun webhooks for Comment-ressourcen.

Vi understøtter webhooks for oprettelse af kommentarer, fjernelse og opdatering.

Hver af disse betragtes som separate hændelser i vores system og har derfor forskellige semantikker
og strukturer for webhook‑hændelserne.

Et vilkårligt antal endpoints kan abonnere på den samme hændelse, fra dashboardet eller via API'en
(se Administrere Webhooks via API'en). Hver webhook leveres uafhængigt.

---