#### Sakai

Sakai supporta la Registrazione Dinamica LTI 1.3 nelle release con LTI Advantage. Dalla **Area di amministrazione**:

1. Accedi come amministratore Sakai e apri la **Administration Workspace**.
2. Scegli **External Tools** > **Install LTI 1.3 Tool**.
3. Incolla l'URL di registrazione FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ottienilo qui</a>) e invia.
4. Approva lo strumento quando l'handshake è completato.

Lo strumento apparirà quindi sotto **External Tools** e può essere aggiunto ai siti dai loro manutentori.

#### Schoology

Le istanze Schoology Enterprise supportano LTI 1.3 ma la disponibilità della Registrazione Dinamica varia in base alla distribuzione. Verifica con il tuo account manager Schoology.

Se la Registrazione Dinamica non è disponibile sulla tua istanza Schoology, dovrai configurare l'integrazione manualmente utilizzando questi endpoint:

- **URL di accesso OIDC**: `https://fastcomments.com/lti/v1p3/login`
- **URL del collegamento di destinazione**: `https://fastcomments.com/lti/v1p3/launch`
- **URL del set di chiavi pubbliche (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **URL di reindirizzamento**: `https://fastcomments.com/lti/v1p3/launch`

Dopo che Schoology ti fornisce un Client ID e un Deployment ID, contatta il supporto FastComments per registrare la configurazione sul tuo tenant.

#### Altre piattaforme LTI 1.3

Qualsiasi LMS che segua la specifica IMS LTI 1.3 Advantage dovrebbe funzionare con lo stesso URL di registrazione (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ottienilo qui</a>). Cerca un'impostazione etichettata "Registrazione Dinamica", "URL di registrazione dello strumento", "endpoint di registrazione per l'inizializzazione dello strumento" o simile.

Se la tua piattaforma supporta solo la configurazione manuale di LTI 1.3, usa i quattro endpoint elencati nella sezione Schoology sopra e contatta il supporto per finalizzare.