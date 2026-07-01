L'SDK FastComments fornisce tre client API:

### PublicAPI - Metodi Sicuri per il Client

Il `PublicAPI` contiene metodi che sono sicuri da chiamare dal codice client‑side (app iOS/macOS). Questi metodi:
- Non richiedono una chiave API
- Possono utilizzare token SSO per l'autenticazione
- Sono soggetti a limitazione di velocità per utente/dispositivo
- Sono adatti per applicazioni rivolte all'utente finale

**Caso d'uso esempio**: Recuperare e creare commenti nella tua app iOS

### DefaultAPI - Metodi Server‑Side

Il `DefaultAPI` contiene metodi autenticati che richiedono una chiave API. Questi metodi:
- Richiedono la tua chiave API FastComments
- Dovrebbero essere chiamati SOLO dal codice server‑side
- Forniscono accesso completo ai tuoi dati FastComments
- Sono soggetti a limitazione di velocità per tenant

**Caso d'uso esempio**: Operazioni amministrative, esportazione massiva di dati, gestione degli utenti

### ModerationAPI - Metodi della Dashboard del Moderatore

Il `ModerationAPI` fornisce una suite completa di API di moderazione in tempo reale e veloci. Ogni metodo `ModerationAPI` accetta un parametro `sso` e può autenticarsi tramite SSO o un cookie di sessione FastComments.com.

**Caso d'uso esempio**: Creare un'esperienza di moderazione per i moderatori della tua comunità

**IMPORTANTE**: Non esporre mai la tua chiave API nel codice client‑side. Le chiavi API dovrebbero essere utilizzate solo lato server.