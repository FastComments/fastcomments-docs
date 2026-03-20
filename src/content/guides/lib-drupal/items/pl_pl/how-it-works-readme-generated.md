Gdy użytkownik odwiedza encję z włączonym polem FastComments:

1. Widget JavaScript FastComments jest ładowany z CDN.
2. Jeśli SSO jest skonfigurowane, tożsamość użytkownika Drupala jest przekazywana do FastComments.
3. Zapasowe rozwiązanie `<noscript>` zapewnia serwerowo renderowane komentarze dla użytkowników bez JavaScript (tylko w trybach Live Comments i Streaming Chat).