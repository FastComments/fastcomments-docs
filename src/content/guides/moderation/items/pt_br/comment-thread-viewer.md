Ao moderar e visualizar encadeamentos de comentários, é desejável poder ir diretamente para um encadeamento para obter contexto durante a moderação.

Isso significa que o fluxo do usuário começa na página de Moderação de Comentários, e então teria que ir de um comentário individual para a página que contém esse comentário, esperar essa página carregar, esperar os comentários carregarem e então rolar até esse comentário.

No entanto, o FastComments oferece uma forma mais rápida. Na página Moderar Comentários, ao lado de cada comentário, há um botão "Visualizar Comentário" no canto inferior direito.

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; title='A Comment' app-screenshot-end]

Se este comentário tiver respostas, o texto do botão exibirá o número de respostas, mas ao clicar ele realiza a mesma ação.

Este botão o levará ao **Visualizador de Encadeamento de Comentários**.

O Visualizador de Encadeamento de Comentários é uma aplicação pequena e de carregamento rápido hospedada pela FastComments que renderiza o encadeamento de comentários da página em que o comentário está e rola até esse comentário.

Isso permite que os moderadores reunam o contexto necessário, rapidamente, sem precisar esperar outra página carregar.