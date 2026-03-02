O plugin suporta três modos de SSO para integrar contas de usuário do Moodle com o FastComments.

#### SSO Seguro (Recomendado)

Os dados do usuário são assinados no servidor usando HMAC-SHA256 com seu API Secret. Esta é a opção mais segura e é recomendada para uso em produção.

Com o SSO Seguro:

- Nomes de usuário, e-mails e avatares são transmitidos com segurança ao FastComments.
- Administradores do site Moodle são sincronizados automaticamente como moderadores do FastComments.
- Os usuários não podem se passar por outras contas.
- Requer que o **API Secret** esteja configurado nas configurações do plugin.

#### SSO Simples

Os dados do usuário (nome, e-mail, avatar) são enviados no cliente sem uma assinatura criptográfica. Isso é mais fácil de configurar, pois não requer um API Secret, mas é menos seguro porque os dados do usuário não são verificados no servidor.

#### Nenhum

Sem integração SSO. Os usuários comentam anonimamente ou precisam fazer login no FastComments separadamente. Use esta opção se você não quiser que as contas de usuário do Moodle estejam vinculadas aos comentários.