Når en bruger besøger en entitet med FastComments-feltet aktiveret:

1. FastComments JavaScript-widget indlæses fra CDN'en.
2. Hvis SSO er konfigureret, overføres brugerens Drupal-identitet til FastComments.
3. Et `<noscript>` fallback leverer server-renderede kommentarer til brugere uden JavaScript (kun i Live Comments- og Streaming Chat-tilstande).