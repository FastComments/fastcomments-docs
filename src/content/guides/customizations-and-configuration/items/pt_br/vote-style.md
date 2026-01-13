[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Por padrão, o FastComments exibirá opções de votação como setas para cima e para baixo, permitindo que os usuários votem a favor ou contra um comentário.

No entanto, é possível alterar o estilo da barra de votação. As opções atuais são os botões padrão Up/Down, ou utilizar um mecanismo de votação no estilo Coração.

Usamos a flag **voteStyle** da seguinte forma:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Recomendamos fortemente que você faça isso sem código, pois também habilita validações no lado do servidor. Na página de personalização do widget, veja a seção "Estilo de Votação".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

A votação também pode ser desativada, veja `Disable Voting` acima das opções de estilo.

---