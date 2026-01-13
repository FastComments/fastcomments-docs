Um `NotificationCount` object representa a contagem de notificações não lidas e os metadados de um usuário.

Se não houver notificações não lidas, não haverá um `NotificationCount` para o usuário.

Objetos `NotificationCount` são criados automaticamente e não podem ser criados via API. Eles também expiram após um ano.

Você pode limpar a contagem de notificações não lidas de um usuário excluindo o seu `NotificationCount`.

A estrutura do objeto `NotificationCount` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // id do usuário
    count: number
    createdAt: string // string de data
    expireAt: string // string de data
}
[inline-code-end]