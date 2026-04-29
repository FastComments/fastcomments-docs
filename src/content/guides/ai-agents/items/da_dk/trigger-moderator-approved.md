Udløses når en moderator godkender en kommentar.

### Kontekst som agenten modtager

- Den nyligt godkendte kommentar.
- Den **udløsende bruger-ID** - moderatoren som godkendte.
- Valgfri tråd-, brugerhistorik- eller sidekontekst som konfigureret.

### Hvem udløser dette

En handling udført af en menneskelig moderator.

### Værd at bemærke

- En "approved" kommentar er en **synlig** kommentar i FastComments-terminologi. Se [Hvordan godkendelser fungerer](/guide-moderation.html#moderation-approvals) i moderationsguiden for forskellen mellem godkendt/ikke-godkendt og gennemgået/ikke-gennemgået.
- Triggeren udløses ved godkendelses **overgange**: en kommentar der går fra ikke-godkendt til godkendt udløser den; en kommentar som allerede var godkendt og blot gemmes igen gør det ikke.
- For lejere hvor kommentarer som standard automatisk godkendes, udløses denne trigger kun, når en moderator eksplicit godkender en tidligere skjult kommentar igen.

### Almindelige anvendelser

- **Velkomst / engagement** - en agent kan svare førstegangs-kommentatorer i det øjeblik en moderator godkender dem, i stedet for ved kommentarens oprettelsestidspunkt.
- **Koordination mellem agenter** - hvis en separat agent havde markeret kommentaren til gennemgang, er godkendelsen signalet om, at den menneskelige gennemgang er færdig.
- **Revisionsspor** via [Webhooks](#webhooks-overview).

---