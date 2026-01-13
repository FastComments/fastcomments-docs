Tous les changements apportés à l'objet Comment dans le système déclenchent un événement qui finit dans une file d'attente.

L'événement webhook initial est généralement envoyé dans les six secondes suivant la survenue de la source de l'événement.

Vous pouvez surveiller cette file d'attente dans l'administration des Webhooks au cas où votre API tomberait en panne.

Si une requête vers votre API échoue, nous la remettrons en file d'attente selon un calendrier.

That schedule is `1 Minute * the retry count`. Si l'appel échoue une fois, il réessaiera dans
une minute. S'il échoue deux fois, il attendra alors deux minutes, et ainsi de suite. Cela permet de
ne pas surcharger votre API si elle tombe en panne pour des raisons liées à la charge.

Les webhooks peuvent être annulés depuis la [page des journaux](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).