The FastComments LTI 1.3 integration segue il principio del privilegio minimo: utilizza solo le claim di launch necessarie per identificare l'utente, associare i commenti al corso e alla risorsa corretti e applicare permessi basati sui ruoli.

Il resto di questa pagina mappa ogni claim consumata dall'integrazione, ogni servizio LTI Advantage che non richiede e ogni categoria di dati che non raccoglie. I revisori di sicurezza e acquisti possono prelevare le risposte direttamente dalle tabelle sottostanti.

## Data Elements Received From the LMS

Every LTI 1.3 launch carries a signed JWT from the LMS. FastComments extracts the following claims from that JWT and uses nothing else:

| Campo | Claim LTI | Scopo | Obbligatorio | Memorizzato |
|-------|-----------|-------|--------------|-------------|
| Identificatore utente | `sub` | Identifica l'utente in modo coerente tra i lancio in modo che la stessa persona corrisponda allo stesso utente SSO di FastComments | Sì | Sì, come parte di un ID SSO interno stabile |
| Nome visualizzato | `name` | Attribuzione mostrata accanto ai commenti dell'utente | Sì (fa fallback a "Utente LMS" se assente) | Sì |
| Email | `email` | Corrispondenza account, notifiche, moderazione, corrispondenza di supporto | Facoltativa (l'integrazione funziona senza) | Sì quando fornita |
| URL avatar | `picture` | Visualizzato nei commenti dell'utente | Facoltativo | Solo URL; FastComments non scarica né ri-ospita l'immagine |
| Ruoli | `https://purl.imsglobal.org/spec/lti/claim/roles` | Determina se l'utente è amministratore, docente (moderatore) o studente | Sì | Flag derivati `isAdmin` / `isModerator` sulla sessione SSO |
| Contesto del corso | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Associa il thread di commenti al corso LMS corretto | Sì | Sì, come parte dell'identificatore di pagina risolto |
| Collegamento alla risorsa | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Associa i commenti all'attività o alla collocazione dello strumento corretta all'interno del corso | Sì quando presente | Sì, come parte dell'identificatore di pagina risolto |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Instrada il launch alla configurazione tenant FastComments corretta | Sì | Sì, nel record di configurazione LTI di FastComments |

## Claims and Scopes Declared at Registration

During LTI 1.3 Dynamic Registration, FastComments registers itself with `scope: ""` (no additional OAuth scopes) and declares only these OpenID Connect claims:

`iss`, `sub`, `name`, `email`, `picture`

It registers two message types:

- `LtiResourceLinkRequest` - the standard course launch into FastComments.
- `LtiDeepLinkingRequest` - allows instructors to place the FastComments tool inside a course.

No additional access tokens are requested from the LMS.

## LTI Advantage Services Not Requested

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | The integration does not need a course roster; user identity arrives with each launch |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | No | The integration is not gradebook-aware |
| Deep Linking beyond the standard placement return | No additional data | Deep linking is used only for instructor placement of the tool; no course content is enumerated |

## Data Not Collected

Beyond LTI itself, FastComments does not request or receive the following from the LMS or user:

| Categoria | Raccolti? |
|----------|-----------|
| Voti degli studenti | No |
| Invii delle assegnazioni | No |
| Registri di presenza | No |
| Elenchi completi del corso | No |
| Identificatori governativi | No |
| Data di nascita | No |
| Indirizzo postale o numero di telefono | No |
| Informazioni finanziarie | No |
| Credenziali amministratore LMS | No |

## Access Boundaries

- FastComments only receives data inside an authorized LTI 1.3 launch signed by the LMS's registered keys. The integration does not call back into the LMS for additional information.
- Launch tokens are single-use and short-lived. Replayed or expired tokens are rejected.
- LMS administrators control where the tool is deployed inside their platform. D2L Brightspace, for example, supports per-deployment org-unit scoping and per-deployment security settings, which allows administrators to restrict the tool to specific courses or org units rather than making it available globally. Moodle, Blackboard, Sakai, and Schoology offer equivalent per-deployment controls in their LTI 1.3 implementations.

## Storage and Retention

FastComments retains LTI-derived data for the duration of the active commenting service and according to customer-configured retention settings. Comment data is stored in encrypted-at-rest production storage. On account termination or written deletion request, FastComments deletes or anonymizes customer data per the applicable agreement.

For full storage and data-handling details, see the <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments Privacy Policy</a>.

## Review Cadence

Any new LTI feature that would require additional claims, scopes, or LTI Advantage services is reviewed before release to confirm the requested access is necessary and proportionate to the feature being shipped.

## Short Statement for Security Questionnaires

> FastComments applies least privilege and data minimization to its LTI 1.3 integration. The integration uses only the LTI launch claims required to authenticate the user (`sub`, `name`, `email`, `picture`), determine their role, and identify the course and resource that comments belong to. FastComments does not request Names and Role Provisioning Services, Assignment and Grade Services, gradebook data, attendance, full rosters, or LMS administrative access. LMS administrators retain control over which org units, courses, and deployments the tool is available in.