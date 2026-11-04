#### Mencionando Usuários em Outros Grupos

Se dois usuários pertencem a dois conjuntos de grupos diferentes, e não há interseção entre eles, não poderão se `@mencionar` mutuamente.

Se um usuário digita manualmente uma `@menção` e envia seu comentário, ela permanecerá como texto simples. O outro usuário não será marcado.

#### Mantendo os Grupos

Os `Groups` são definidos usando os recursos de API `Pages` e `SSOUsers`, respectivamente.

A API `Pages` pode ser invocada para definir o conjunto de grupos com permissão para acessar a página. Por padrão, todos os grupos, e usuários que não pertencem a um grupo, têm acesso.

Da mesma forma, a API `SSOUsers` pode ser invocada para definir os grupos associados a cada usuário.

Para ambos os recursos, não há limitações quanto a quando os grupos podem ser definidos ou atualizados.

Se o objetivo for apenas limitar que os usuários se `@mencionem` mutuamente, então não é necessário levar os `Pages` em consideração.

##### Nota!

Definir e atualizar os grupos de usuários SSO não requer o uso da API, e pode ser feito automaticamente definindo os ids dos grupos no payload SSO passado para o widget de comentários. No entanto, para listas grandes de grupos, isso não é recomendado, pois o usuário teria que enviar esse payload a cada carregamento de página.