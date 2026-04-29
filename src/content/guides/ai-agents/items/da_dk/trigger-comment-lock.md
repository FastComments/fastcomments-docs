---
Udløses når en kommentar låses.

### Kontekst agenten modtager

- Den låste kommentar.
- Valgfri tråd / brugerhistorik / sidekontekst som konfigureret.

### Hvem udløser dette

- En moderator, der bruger låsehandlingen på moderationssiden eller kommentar-widgeten.

### Almindelige anvendelser

- **Underret moderatorer** - en låsebegivenhed følger ofte en ophedet tråd; et webhook ud til din moderations-Slack-kanal kan lade mennesker tage resten.
- **Håndhævelse af nedkøling** - planlæg en [udskudt trigger](#trigger-deferred-delay) på en separat agent, som 24 timer efter låsningen overvejer, om den skal oplåses.

### Par

Se [Trigger: Kommentar oplåst](#trigger-comment-unlock) for den spejlede udløser.

---