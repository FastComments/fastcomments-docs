Como os links de login são essencialmente senhas, levamos a segurança muito a sério.

Todos os links de login em nosso sistema são configurados para expirar após um determinado período de tempo, e também temos mecanismos para detectar
a adivinhação de um link de login. Alguns links de login são divididos em múltiplas senhas e, se uma for adivinhada,
a outra será invalidada.

### Segurança em comparação com senhas

Na maioria dos sistemas que exigem uma senha, você pode passar por um mecanismo de "Esqueci minha senha"
se tiver o e-mail do usuário. Isso significa que, se você tiver acesso à conta de e-mail do usuário,
não importa se o sistema sob ataque usa senhas ou links mágicos.

### Alertas de novo login por IP

Quando um login ocorre a partir de um endereço IP que não foi visto antes para uma determinada conta, o FastComments envia um e-mail de alerta de segurança
com a localização aproximada e o endereço IP. Isso ajuda os usuários a detectar acessos não autorizados. Observe que o FastComments não armazena
endereços IP brutos — apenas uma forma ofuscada é armazenada para fins de segurança.

### Segurança em comparação com MFA

Links de login são menos seguros do que MFA. O FastComments agora oferece suporte à autenticação de dois fatores (2FA)
para contas de administrador para fornecer maior segurança. Quando a 2FA está ativada, ela é exigida mesmo ao usar links de login.