Udløser agenten hver gang en ny kommentar bliver postet på en side omfattet af agentens [scope](#scope-url-locale).

### Kontekst agenten modtager

- Den nye kommentar i sin helhed - text, author, votes, parent ID, page URL ID.
- Valgfrit: parent comment og prior replies i samme tråd, hvis [thread context](#context-options) er slået til.
- Valgfrit: commenterens trust factor, account age, ban history, og recent comments, hvis [user history context](#context-options) er slået til.
- Valgfrit: page metadata, hvis [page context](#context-options) er slået til.

### Bemærk

- Udløseren affyres **efter** kommentaren er blevet gemt. Agenten kan henvise til den direkte ved kald til værktøjer.
- Den affyres **ikke** for kommentarer skrevet af en anden agent i samme tenant.
- Den affyres for både verificerede og ikke-verificerede kommentarer. Hvis din tenant kræver moderatorgodkendelse, før en kommentar er synlig (se [Hvordan godkendelser fungerer](/guide-moderation.html#moderation-approvals) i moderation-guiden), udløses triggeren, når kommentaren oprettes, ikke når den senere godkendes. Moderator-botten kan instrueres til at godkende kommentarer for dig efter gennemgang.

### Almindelige anvendelser

- **Moderering** - tjek kommentaren op imod fællesskabets retningslinjer, markér spam eller advær førstegangsbrugere.
- **Velkomsthilsen** - selvom [Trigger: New User First Comment](#trigger-new-user-first-comment) normalt er et bedre match til hilsner, da den affyres én gang pr. bruger.
- **Trådopsummering** - normalt parret med en [trigger delay](#trigger-deferred-delay), så tråden roer sig, før agenten kører.

---