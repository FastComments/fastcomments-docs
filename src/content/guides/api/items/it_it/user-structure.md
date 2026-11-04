`User` è un oggetto che rappresenta il denominatore comune di tutti gli utenti.

Tieni presente che in FastComments abbiamo diversi casi d'uso per gli utenti:

- Secure SSO
- Simple SSO
- Utenti Tenant (Ad esempio: Amministratori)
- Commentatori

Questa API è per i **Commentatori** e per gli utenti creati tramite **Simple SSO**. In pratica, qualsiasi utente creato
tramite il tuo sito può essere accessibile tramite questa API. Anche gli Utenti Tenant possono essere recuperati in questo modo, ma otterrai più informazioni interagendo con l'API `/tenant-users/`.

Per `Secure SSO` utilizza l'API `/sso-users/`.

Non puoi aggiornare questi tipi di utenti. Hanno creato il loro account tramite il tuo sito, quindi forniamo un accesso di base in sola lettura, ma
non puoi apportare modifiche. Se desideri avere questo tipo di flusso - devi configurare `Secure SSO`.

La struttura dell'oggetto `User` è la seguente:

[inline-code-attrs-start title = 'Struttura di User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Questo è anche l'id usato come userId sugli oggetti commento. **/
    id: string
    username: string
    /** Un link al blog del commentatore, per esempio. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]