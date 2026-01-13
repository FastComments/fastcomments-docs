#### Mencionando Usuários em Outros Grupos

Se dois usuários pertencem a dois conjuntos diferentes de grupos, e não há interseção, eles não poderão `@mention` um ao outro.

Se um usuário digitar manualmente um `@mention` e enviar seu comentário, ele permanecerá como texto simples. O outro usuário não será marcado.

#### Maintaining the Groups

`Groups` are defined using the `Pages` and `SSOUsers` API resources, respectively.

The `Pages` API can be invoked to define the set of groups allowed to access the page. By default, all groups, and users that do
not belong to a group, have access.

Similarly, the `SSOUsers` API can be invoked to define the groups associated with each user.

For both resources, there are no limitations as to when the groups can be set or updated.

If it's only desired to limit users from `@mention`'ing each other, then `Pages` do not have to be taken into consideration.

##### Observação!

Definir e atualizar os grupos de usuários SSO não requer o uso da API, e pode ser atualizado automaticamente ao definir o
group ids no payload SSO passado para o widget de comentários. No entanto, para listas grandes de grupos, isso não é recomendado, pois o usuário
teria que enviar esse payload a cada carregamento de página.