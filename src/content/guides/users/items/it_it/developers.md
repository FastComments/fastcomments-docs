---
Per gli sviluppatori che potresti non voler impostare come `Administrators`, considera di creare un utente `Administrator`
con le seguenti autorizzazioni:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Questo insieme di autorizzazioni fornirà a uno sviluppatore tutto il necessario per configurare FastComments nonché
la visibilità nel sistema richiesta per assicurarsi che funzioni.

La motivazione per queste autorizzazioni è la seguente:

1. **Analytics Admin**: Questo ruolo può essere usato per diagnosticare l'utilizzo di FastComments.
2. **Customizations Admin**: Questo sarà necessario per applicare lo stile personalizzato al widget dei commenti.
3. **Data Management Admin**: Questo sarà necessario per gestire importazioni ed esportazioni, e configurare i webhook.
4. **Comment Moderation Admin**: Questo sarà necessario per visualizzare i dati dei commenti, almeno durante la configurazione.
5. **API/SSO Admin**: Questo permetterà loro di recuperare le API keys direttamente dalla nostra piattaforma. Consideriamo
questo più sicuro rispetto a un `Administrator` che le copia per loro e invia l'API Secret tramite email, il che
   potrebbe non essere molto sicuro.
---