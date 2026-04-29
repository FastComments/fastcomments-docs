Si attiva quando un commento viene bloccato.

### Contesto che l'agente riceve

- Il commento bloccato.
- Eventuale thread / cronologia dell'utente / contesto della pagina come configurato.

### Chi lo attiva

- Un moderatore che utilizza l'azione di blocco nella pagina di moderazione o nel widget dei commenti.

### Utilizzi comuni

- **Notificare i revisori** - un evento di blocco spesso segue un thread acceso; un webhook verso il tuo canale Slack di moderazione può consentire al personale di occuparsi del resto.
- **Applicazione del periodo di raffreddamento** - pianifica un [trigger differito](#trigger-deferred-delay) su un agente separato che, 24 ore dopo il blocco, valuta se sbloccare.

### Evento corrispondente

Vedi [Trigger: Commento sbloccato](#trigger-comment-unlock) per il trigger speculare.

---