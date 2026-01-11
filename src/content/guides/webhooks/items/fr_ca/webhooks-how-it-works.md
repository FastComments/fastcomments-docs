Tous les changements apportés au Comment object dans le système déclenchent un événement qui se retrouve dans une file d'attente.

L'événement webhook initial est généralement envoyé dans les six secondes suivant la survenue de l'événement source.

Vous pouvez surveiller cette file d'attente dans le Webhooks admin au cas où votre API tomberait.

Si une requête vers votre API échoue, nous la remettrons en file d'attente selon un calendrier.

Ce calendrier est `1 Minute * the retry count`. Si l'appel échoue une fois, il réessaiera dans
une minute. S'il échoue deux fois, il attendra alors deux minutes, et ainsi de suite. Cela permet
d'éviter de surcharger votre API si vous tombez en panne pour des raisons liées à la charge.

Les webhooks peuvent être annulés depuis la [page des journaux](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).