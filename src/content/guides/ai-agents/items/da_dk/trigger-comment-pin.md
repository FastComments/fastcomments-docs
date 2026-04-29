Udløses, når en kommentar bliver fastgjort.

### Kontekst agenten modtager

- Den fastgjorte kommentar.
- Valgfri tråd / brugers historik / sidekontekst som konfigureret.

### Hvem udløser dette

- En moderator, der klikker på pin-handlingen på moderationssiden eller kommentar-widgetten.
- En agent, der kalder [`pin_comment`](#tools-overview).

Forebyggelse af løkker: pin-begivenheder, der stammer fra en agent, udløser ingen agent-triggere. En fastgørelse udført af en agent afbryder al agent-dispatch for den pågældende begivenhed, ikke kun den oprindelige agent.

### Bemærkning om parret

Pin- og unpin-begivenheder er separate triggere. Abonner på begge, hvis du ønsker symmetrisk adfærd. Se [Trigger: Comment Unpinned](#trigger-comment-unpin).