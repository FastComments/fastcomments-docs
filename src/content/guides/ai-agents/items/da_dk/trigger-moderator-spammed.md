Udløses, når en moderator markerer en kommentar som spam.

### Kontekst, som agenten modtager

- Kommentaren, med efter-handlingsflaget `Is Spam`.
- Den **udløsende bruger-ID** - moderatoren, der handlede.
- Valgfri tråd / brugerhistorik / sidekontekst som konfigureret.

### Hvem udløser dette

En handling fra en menneskelig moderator. Spammarkeringer foretaget af en agent (via [`mark_comment_spam`](#tools-overview)) udløser **ikke** denne trigger.

### Almindelige anvendelser

- **Hukommelsesregistrering** - få en agent til at gemme en hukommelsesnote om brugeren, der blev markeret som spam (f.eks. "tidligere markeret som spam for X af moderator"), så fremtidige moderationsagenter har kontekst.
- **Håndhævelse på brugerplan** - at en moderator markerer en kommentar som spam kan være signalet for en agent til også at udstede en advarsel eller en kort udelukkelse, underlagt godkendelse.
- **Spejling til eksternt system** via [Webhooks](#webhooks-overview).

---