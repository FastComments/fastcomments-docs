## Troubleshooting

**"U heeft geen toestemming" bij het verbinden.** De aangemelde gebruiker is geen API-beheerder op het account.  
Vraag de accounteigenaar om de API-rechten toe te kennen op de Users‑pagina, of verbind als de eigenaar.

**De verbinding is gelabeld met de verkeerde site.** De toestemmingspagina verbindt met het account waarmee u op dat moment bent aangemeld. Ontkoppel in Zapier, wissel van account in het FastComments‑dashboard, en verbind opnieuw.

**Evenementen komen niet meer binnen.** Controleer de Webhooks‑pagina in het dashboard. Een abonnement waarvan het eindpunt zes dagen achter elkaar faalde, wordt automatisch uitgeschakeld en toont de reden. Schakel het daar opnieuw in, of zet de Zap uit en weer aan. Als het abonnement volledig ontbreekt, heeft iemand het verwijderd; het uit‑ en aanzetten van de Zap maakt het opnieuw aan.

**Zapier zegt dat het account opnieuw moet worden verbonden.** De verbinding is ingetrokken vanaf de Connected Apps‑pagina, de gebruiker die het heeft goedgekeurd verloor de API‑toestemming, of het account is verwijderd. Maak opnieuw verbinding vanuit Zapier.

**Een actie mislukt met "heeft geen schrijfrechten".** De verbinding was goedgekeurd met alleen‑lezen rechten. Maak opnieuw verbinding en keur beide rechten goed.

**Rate limits en credits.** Acties en zoekopdrachten gebruiken API‑credits van uw abonnement en vallen onder dezelfde rate limits als de REST API. Triggers gebruiken geen credits. Een Zap die tegen een limiet aanloopt, wordt door Zapier opnieuw geprobeerd na de vertraging die FastComments meldt.

**De vervolgkeuzelijst Domein is leeg.** Domeinen verschijnen zodra ze zijn geconfigureerd op de Domains‑pagina in het FastComments‑dashboard. Laat het veld leeg om gebeurtenissen voor elk domein te ontvangen.