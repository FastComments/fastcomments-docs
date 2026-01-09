---
Como os links de login são essencialmente senhas, levamos a segurança muito a sério.

Todos os links de login em nosso sistema são configurados para expirar após um determinado período de tempo, e também temos mecanismos em vigor para detectar
a adivinhação de um link de login. Alguns links de login são divididos em várias senhas, e se uma for adivinhada,
a outra será invalidada.

### Segurança em comparação com senhas

Com a maioria dos sistemas que exigem uma senha, você pode passar por um mecanismo de "Esqueci a senha"
se você tiver o e-mail do usuário. Isso significa que, se você tiver acesso à conta de e-mail do usuário,
não importa se o sistema sob ataque usa senhas ou links mágicos.

### Segurança em comparação com MFA

Links de login são menos seguros que o MFA. FastComments agora suporta autenticação de dois fatores (2FA)
para contas de administrador para fornecer segurança aprimorada. Quando o 2FA está habilitado, ele é exigido mesmo ao usar links de login.

---