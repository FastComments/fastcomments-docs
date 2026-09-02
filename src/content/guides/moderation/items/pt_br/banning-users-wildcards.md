É possível banir usuários que utilizam determinados provedores de email usando curingas.

Por exemplo, se você descobrir que todos os comentários de **@bademail.com** são spam, pode simplesmente banir todo o provedor de email inserindo "*@bademail.com" no campo de email ao adicionar um usuário banido.

Observe o "*" antes do @ no email.

### Subdomínios

Um banimento de domínio também cobre todos os subdomínios desse domínio. Banir `*@bademail.com` também bane `someone@mail.bademail.com` e `someone@eu.mail.bademail.com`, portanto não há necessidade de adicionar um banimento separado para cada subdomínio.

Se você quiser banir apenas um subdomínio específico, insira esse subdomínio em vez disso, por exemplo `*@mail.bademail.com`. Esse banimento não afeta `someone@bademail.com`.

### Banindo um Domínio a partir de um Comentário

Você não precisa digitar o padrão manualmente. Quando você bane um usuário a partir de um comentário na página Moderar Comentários, a caixa de diálogo de banimento possui a opção “Ban All @domain Users” que cria o mesmo banimento `*@domain` para o domínio de email do comentarista.

### Padrões Suportados

A única forma de curinga suportada é um único `*` no lugar da parte inteira do nome, seguido por `@` e um domínio. Outras formas são rejeitadas ao tentar salvá‑las:

- `*@*.bademail.com` não é necessário, pois `*@bademail.com` já cobre subdomínios.
- `name*@bademail.com` e `*bademail.com` não são suportados.