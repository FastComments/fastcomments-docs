Wenn ein Benutzer eine Entität besucht, bei der das FastComments-Feld aktiviert ist:

1. Das FastComments JavaScript-Widget wird vom CDN geladen.
2. Wenn SSO konfiguriert ist, wird die Drupal-Identität des Benutzers an FastComments übermittelt.
3. Ein `<noscript>`-Fallback liefert serverseitig gerenderte Kommentare für Benutzer ohne JavaScript (nur in den Modi Live Comments und Streaming Chat).