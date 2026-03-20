Ko uporabnik obišče entiteto s omogočenim poljem FastComments:

1. JavaScript gradnik FastComments se naloži iz CDN.
2. Če je SSO konfiguriran, se uporabnikova Drupal identiteta posreduje FastComments.
3. Nadomestek `<noscript>` zagotavlja komentarje, upodobljene na strežniku, za uporabnike brez JavaScripta (samo v načinih Live Comments in Streaming Chat).