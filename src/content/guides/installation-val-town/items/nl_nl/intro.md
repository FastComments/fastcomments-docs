---
[Val Town](https://val.town) draait TypeScript op Deno, dus een val is een echte server. Dat maakt het een goede match voor FastComments: de widget is een script‑tag op de pagina, en alles wat een geheim nodig heeft, zoals Secure SSO of het verifiëren van een webhook, kan server‑side draaien in dezelfde val.

Deze gids behandelt het toevoegen van de commentaarwidget aan een HTTP val, het tonen van commentaartellingen op een indexpagina, het aanmelden van gebruikers met het Val Town‑account dat ze al hebben, en het ontvangen van commentaar‑webhooks.

Je hebt geen account nodig om het te proberen. De voorbeelden gebruiken `tenantId: "demo"`, een gedeelde sandbox, en Stap 2 behandelt het overschakelen naar je eigen.
---