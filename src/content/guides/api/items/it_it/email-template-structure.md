Un oggetto `EmailTemplate` rappresenta la configurazione per un modello di posta elettronica personalizzato, per un tenant.

Il sistema selezionerà il modello di posta elettronica da utilizzare tramite:

- Il suo identificatore di tipo, che chiamiamo `emailTemplateId`. Sono costanti.
- The `domain`. Cercheremo prima di trovare un template per il dominio a cui è legato l'oggetto correlato (come un `Comment`), e se non viene trovata una corrispondenza proveremo a trovare un template in cui domain è null o `*`.

La struttura per l'oggetto `EmailTemplate` è la seguente:

[inline-code-attrs-start title = 'Struttura del modello di posta elettronica'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** SOLO LETTURA **/
    createdAt: string
    /** SOLO LETTURA **/
    updatedAt: string
    /** SOLO LETTURA **/
    updatedByUserId: string
    /** Il dominio a cui il template dovrebbe essere associato. **/
    domain?: string | '*' | null
    /** Il contenuto del template email in sintassi EJS. **/
    ejs: string
    /** Una mappa delle chiavi di traduzione sovrascritte ai valori, per ogni locale supportato. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Un oggetto che rappresenta il contesto di rendering del template. **/
    testData: object
}
[inline-code-end]

### Note

- Puoi ottenere i valori validi di `emailTemplateId` dall'endpoint `/definitions`.
- L'endpoint `/definitions` include anche le traduzioni predefinite e i dati di test.
- I template non verranno salvati se la struttura o i dati di test non sono validi.

---