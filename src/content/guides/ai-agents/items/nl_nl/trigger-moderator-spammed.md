---
Wordt geactiveerd wanneer een moderator een reactie als spam markeert.

### Context die de agent ontvangt

- De reactie, met de post-actievlag `Is Spam`.
- De **triggerende gebruiker-ID** - de moderator die handelde.
- Optionele thread / gebruikersgeschiedenis / paginacontext zoals geconfigureerd.

### Wie activeert dit

Een handeling door een menselijke moderator. Door agenten veroorzaakte spammarkeringen (via [`mark_comment_spam`](#tools-overview)) activeren deze trigger **niet**.

### Veelvoorkomende toepassingen

- **Geheugenregistratie** - laat een agent een geheugenopmerking opslaan over de gespamde gebruiker (bijv. "eerder gespamd voor X door moderator") zodat toekomstige moderatie-agents context hebben.
- **Handhaving op gebruikersniveau** - het markeren van een reactie als spam door een moderator kan voor een agent aanleiding zijn om ook een waarschuwing of een korte ban uit te delen, afhankelijk van goedkeuring.
- **Spiegel naar extern systeem** via [Webhooks](#webhooks-overview).

---