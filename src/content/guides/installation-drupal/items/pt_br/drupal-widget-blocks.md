---
O módulo fornece vários blocos que você pode colocar a partir de `Structure > Block layout` (`/admin/structure/block`).

- **FastComments Widget** - O widget principal de comentários. Detecta automaticamente a entidade atual. Ele ignorará entidades que já tenham o FastComments field anexado, então você não verá widgets duplicados na mesma página.
- **FastComments Live Chat** - Chat de streaming em tempo real. Pode ser colocado ao lado do campo de comentários na mesma página.
- **FastComments Collab Chat** - Anotação e discussão por seleção de texto.
- **FastComments Image Chat** - Anotação baseada em coordenadas em imagens. Os visitantes clicam em uma imagem para deixar comentários vinculados a locais específicos.
- **FastComments Recent Comments** - Exibe comentários recentes em todo o seu site. A contagem é configurável no bloco.
- **FastComments Top Pages** - Mostra as páginas do seu site com mais comentários.

Os blocos centrados em conteúdo (Live Chat, Collab Chat, Image Chat) detectam automaticamente a entidade atual e recorrem a um identificador baseado em caminho em páginas que não correspondem a uma entidade. Isso significa que eles funcionam em páginas de taxonomia, views e rotas personalizadas sem qualquer configuração adicional.

---