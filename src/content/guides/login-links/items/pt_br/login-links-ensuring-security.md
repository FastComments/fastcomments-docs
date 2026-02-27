Como os links de login são essencialmente senhas, levamos a segurança muito a sério.

Todos os links de login em nosso sistema são definidos para expirar após um determinado período de tempo, e também temos mecanismos em vigor para detectar
a adivinhação de um link de login. Alguns links de login são divididos em múltiplas senhas, e se uma for descoberta,
a outra será invalidada.

### Segurança em comparação com senhas

Na maioria dos sistemas que exigem uma senha, você pode passar por um mecanismo de 'Esqueci a senha'
se tiver o e-mail do usuário. Isso significa que, se você tiver acesso à conta de e-mail do usuário,
não importa se o sistema sob ataque usa senhas ou links mágicos.

### Alertas de login de novo IP

Quando um login ocorre a partir de um endereço IP que não havia sido visto antes para uma dada conta, FastComments envia um e-mail de alerta de segurança
com a localização aproximada e o endereço IP. Isso ajuda os usuários a detectar acessos não autorizados. Observe que o FastComments não armazena
endereços IP brutos — apenas uma forma ofuscada é armazenada por motivos de segurança.

### E-mail de backup para recuperação de conta

Se você perder o acesso ao seu e-mail primário, pode usar um e-mail de backup verificado para recuperar sua conta. Seu e-mail de backup funciona
com todos os fluxos de login. Você pode digitá-lo na página de 'esqueci o nome de usuário', usá-lo com o login por link mágico, ou inseri-lo no
campo de nome de usuário/e-mail para login por senha.

Para configurar um e-mail de backup, vá para [Detalhes da Conta](https://fastcomments.com/auth/my-account/edit-details) e clique
**Definir um E-mail de Backup**. Seu e-mail de backup é usado apenas para recuperação de conta e não receberá notificações.

### Segurança em comparação com MFA

Links de login são menos seguros do que o MFA. O FastComments agora oferece suporte à autenticação de dois fatores (2FA)
para contas de administrador, para fornecer segurança aprimorada. Quando o 2FA está habilitado, ele é exigido mesmo ao usar links de login.