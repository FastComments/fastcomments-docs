Standaard kan elke gebruiker tot `5 comments` in dezelfde minuut indienen.

Dit wordt bijgehouden op basis van user id, anon user id en ip address (gehasht).

Dit kan zonder code worden aangepast op de pagina voor het aanpassen van de widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Houd er rekening mee dat als je de comment creation API gebruikt, je mogelijk het oorspronkelijke `ip` address van de gebruiker wilt meesturen in het verzoek naar onze backend, zodat rate limiting per gebruiker wordt toegepast en niet globaal op je account.