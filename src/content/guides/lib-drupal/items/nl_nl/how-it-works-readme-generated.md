Wanneer een gebruiker een entiteit bezoekt met het FastComments-veld ingeschakeld:

1. Het FastComments JavaScript-widget wordt geladen vanaf het CDN.
2. Als SSO is geconfigureerd, wordt de Drupal-identiteit van de gebruiker doorgegeven aan FastComments.
3. Een `<noscript>` fallback levert server-gerenderde reacties voor gebruikers zonder JavaScript (alleen in de Live Comments- en Streaming Chat-modi).