Com o Controle de Acesso SSO do FastComments, às vezes chamado de RBAC, usuários podem ser restritos a acessar apenas certas páginas ou threads de comentários. Além disso, os usuários só podem `@mention` uns aos outros no mesmo grupo.

## Em Detalhe

Podemos colocar `Users` e, opcionalmente, `Pages` em grupos.

Quando `Users` são colocados em grupos, e `Limit Comments by SSO User Groups` está ativado em Configurações do Widget, então os usuários
verão apenas comentários de usuários em seus mesmos grupos e só poderão `@mention` usuários nos mesmos grupos.

Além disso, `Pages` podem ser colocadas em grupos, e então os usuários só poderão acessar comentários de páginas às quais eles têm acesso.

Chamamos isso de grupos "User-Level" versus grupos "Page-Level".

Então, qual é o certo para você?

#### Use Grupos no nível do usuário se...

- Você quer usar o mesmo `urlId` (URL da página ou ID do artigo), mas restringir comentários por grupo.
- Por exemplo, você quer ter grupos "Novo Usuário" e "Usuário Veterano", e eles nunca devem ver os comentários uns dos outros nas mesmas páginas.

#### Use Grupos no nível da página se...

- Seus grupos têm páginas específicas.
- Por exemplo, usuários no grupo "Páginas Públicas" nunca devem ver artigos "Top Secret".

---