#### Sakai

Sakai supporta LTI 1.3 Dynamic Registration nelle release con LTI Advantage. Dalla **Administration Workspace**:

1. Accedi come amministratore Sakai e apri la **Administration Workspace**.
2. Seleziona **External Tools** > **Install LTI 1.3 Tool**.
3. Incolla l'URL di registrazione di FastComments e invia.
4. Approva lo strumento quando l'handshake è completato.

Lo strumento apparirà quindi sotto **External Tools** e potrà essere aggiunto ai siti dai rispettivi manutentori.

#### Schoology

Le istanze Schoology Enterprise supportano LTI 1.3 ma la disponibilità di Dynamic Registration varia a seconda della distribuzione. Verifica con il tuo account manager Schoology.

Se Dynamic Registration non è disponibile sulla tua istanza Schoology, dovrai configurare l'integrazione manualmente utilizzando questi endpoint:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Dopo che Schoology ti avrà fornito un Client ID e un Deployment ID, contatta il supporto FastComments per registrare la configurazione sul tuo tenant.

#### Other LTI 1.3 Platforms

Qualsiasi LMS che segue la specifica IMS LTI 1.3 Advantage dovrebbe funzionare con lo stesso registration URL. Cerca un'impostazione etichettata "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", o simile.

Se la tua piattaforma supporta solo la configurazione manuale di LTI 1.3, usa i quattro endpoint elencati nella sezione Schoology sopra e contatta il supporto per finalizzare.