Un oggetto `NotificationCount` rappresenta il conteggio delle notifiche non lette e i metadati per un utente.

Se non ci sono notifiche non lette, non esisterà alcun `NotificationCount` per l'utente.

Gli oggetti `NotificationCount` vengono creati automaticamente e non possono essere creati tramite l'API. Scadono inoltre dopo un anno.

Puoi azzerare il conteggio delle notifiche non lette di un utente eliminando il relativo `NotificationCount`.

La struttura dell'oggetto `NotificationCount` è la seguente:

[inline-code-attrs-start title = 'Struttura di NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // id utente
    count: number
    createdAt: string // stringa della data
    expireAt: string // stringa della data
}
[inline-code-end]

---