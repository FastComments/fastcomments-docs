Una volta disattivato il tenant `demo`, il widget potrebbe rifiutarsi di caricarsi con un errore di autorizzazione. Questo accade perché FastComments non sa che dovrebbe consentire l'uso del tuo account su quel dominio.

[Vai qui per aggiungere il tuo sito al tuo account.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town merita un secondo sguardo qui, perché un val può essere raggiungibile da più di un nome host:

- Ogni val HTTP ha un endpoint predefinito lungo, `<org>--<id>.web.val.run`.
- Reclamare un sottodominio personalizzato aggiunge `<name>.val.run`.
- Un [dominio personalizzato](https://docs.val.town/vals/http/custom-domains/) ne aggiunge un terzo.
- I rami ottengono i propri URL.

Aggiungi i nomi host da cui effettivamente servi il widget. Se reclami un sottodominio dopo aver configurato tutto, aggiungilo anche, altrimenti il widget funziona sull'URL vecchio e fallisce su quello nuovo.