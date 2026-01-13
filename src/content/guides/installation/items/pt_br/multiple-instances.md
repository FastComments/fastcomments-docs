Cada instância do widget de comentários é isolada. Por isso, FastComments suporta nativamente mais de uma instância por página, ou múltiplas instâncias apontando para a mesma thread de chat.

No caso da biblioteca VanillaJS, por exemplo, você simplesmente precisa vincular o widget de comentários a diferentes nós DOM. Se você quer simplesmente atualizar a thread atual na página, veja [Trocar Threads de Comentários Sem Recarregar a Página](guide-customizations-and-configuration.html#switching-comment-threads);

### Sincronizando Estado de Autenticação Entre Múltiplas Instâncias

Vamos analisar o exemplo de uma aplicação single-page personalizada que é uma lista de perguntas frequentes com sua própria thread de comentários.

Neste caso, temos múltiplas instâncias de FastComments no DOM ao mesmo tempo.

Isso é ok, mas apresenta alguns desafios para a experiência do usuário.

Considere este fluxo:

1. O usuário visita a página com uma lista de perguntas, cada uma com seu próprio widget de comentários.
2. O usuário insere seu nome de usuário e email e deixa uma pergunta em uma das threads.
3. Ele vê outro item de FAQ sobre o qual tem uma pergunta.
4. Ele vai comentar novamente. Ele precisa inserir seu email e nome de usuário novamente?

Neste caso, FastComments cuida da sincronização do estado de autenticação entre instâncias do widget para você. No passo quatro, o usuário já estará temporariamente autenticado já que inseriu seu nome de usuário e email na mesma página.
