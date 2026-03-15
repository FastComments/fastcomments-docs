Quando un utente visita un'entità con il campo FastComments abilitato:

1. Il widget JavaScript di FastComments viene caricato dal CDN.
2. Se SSO è configurato, l'identità Drupal dell'utente viene passata a FastComments.
3. Un fallback `<noscript>` fornisce commenti renderizzati dal server per gli utenti senza JavaScript (solo nelle modalità Live Comments e Streaming Chat).