Un oggetto `Moderator` rappresenta la configurazione per un moderatore.

Ci sono tre tipi di moderatori:

1. Utenti amministratori che hanno il flag `isCommentModeratorAdmin`.
2. Utenti SSO con il flag `isCommentModeratorAdmin`.
3. Commentatori normali, o utenti FastComments.com, che vengono invitati come Moderatori.

La struttura `Moderator` viene usata per rappresentare lo stato di moderazione nel caso d'uso `3`.

Se vuoi invitare un utente a diventare moderatore tramite l'API, usa l'API `Moderator` creando un `Moderator` e `inviting` them.

Se l'utente non ha un account FastComments.com, l'email di invito li aiuterà a configurarsi. Se hanno già un account, verrà loro concesso l'accesso di moderazione al tuo tenant e il `userId` dell'oggetto `Moderator` verrà aggiornato per puntare al loro utente. Non avrai accesso API al loro utente, poiché in questo caso l'account appartiene a loro ed è gestito da FastComments.com.

Se necessiti della gestione completa dell'account dell'utente, raccomandiamo di usare SSO oppure aggiungerli come [Utente del tenant](https://fastcomments.com/auth/my-account/users) e poi aggiungere un oggetto `Moderator` per tracciare le loro statistiche.

La struttura `Moderator` può essere usata come meccanismo di tracciamento delle statistiche per i casi d'uso `1` e `2`. Dopo aver creato l'utente, aggiungi un oggetto `Moderator` con il loro `userId` definito e le loro statistiche verranno tracciate nella [Pagina Moderatori dei Commenti](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

La struttura dell'oggetto `Moderator` è la seguente:

[inline-code-attrs-start title = "Struttura dell'oggetto Moderator"; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]

---