Para SSO, há a seguinte configuração a considerar para notificações:

- Se o usuário optou por receber notificações.
  - Isso é feito definindo a flag `optedInNotifications` como `true` ou `false` no objeto `SSOUser`.
  - Isso pode ser definido via API.
  - Além disso, se você passar um valor para essa flag no payload, o valor será atualizado automaticamente quando o usuário carregar uma thread de comentários.
- Se o usuário optou por notificações de **assinatura**.
  - Isso é feito definindo a flag `optedInSubscriptionNotifications` como `true` ou `false` no objeto `SSOUser`.
  - Isso pode ser definido via API.
  - Além disso, se você passar um valor para essa flag no payload, o valor será atualizado automaticamente quando o usuário carregar uma thread de comentários.
- Definição do e-mail.
  - Se não estiver presente, não podemos enviar notificações por e-mail.
- Se deve desabilitar os links de cancelamento de inscrição nos e-mails.
  - Isso é feito através da flag `disableUnsubscribeLinks` no objeto `Tenant`.
  - Isso pode ser definido via API.
- Se deve usar um link de cancelamento de inscrição personalizado.
  - Isso é feito por meio da propriedade `footerUnsubscribeURL` no objeto `DomainConfig`.
  - Isso pode ser definido via API.
  - Você também pode considerar definir os cabeçalhos de cancelamento de inscrição relevantes via `emailHeaders` no mesmo objeto.