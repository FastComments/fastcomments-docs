---
Standaard kan elke gebruiker tot `5 comments` indienen binnen dezelfde minuut.

Dit wordt bijgehouden op basis van user id, anon user id, en ip address (hashed).

Dit kan zonder code worden aangepast op de widget‑aanpassingspagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Max comments per minute-veld op de widget‑aanpassingspagina, standaard ingesteld op 5'; title='Beperken van commentaarvolume per gebruiker' app-screenshot-end]

Merk op dat als je de comment creation API gebruikt, je mogelijk het oorspronkelijke `ip`‑adres van de gebruiker wilt doorgeven in het verzoek aan onze backend, zodat rate limiting wordt toegepast
per gebruiker en niet globaal op je account.

---