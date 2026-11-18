As enquetes funcionam da mesma forma que no widget web e são ativadas nas mesmas configurações do widget no seu painel FastComments: `pollCreation` (quem pode anexar uma enquete: desativado, administradores e moderadores, ou todos) e `pollVoting` (usuários autenticados ou qualquer pessoa).

- Comentários com uma enquete a exibem abaixo do texto. Os leitores votam tocando em uma opção, podem mostrar ou ocultar os resultados antes de votar e podem alterar seu voto. As contagens são atualizadas em tempo real conforme outros votam.
- As enquetes podem fechar em uma data, ocultar os resultados até que você vote e tornar os eleitores visíveis para administradores ou para todos ("Ver quem votou").
- Quando o visualizador pode criar enquetes, a entrada de comentários do `FastCommentsView` mostra um botão de enquete que abre o construtor de enquetes. Uma enquete é publicada junto com um comentário, portanto o texto do comentário é obrigatório. A entrada do chat ao vivo omite o botão, como no chat ao vivo da web.

Votar exige um usuário autenticado, então use [SSO](#authentication-sso-readme-generated). O servidor vincula um voto anônimo a um cookie de sessão, que o cliente da API não mantém, portanto, com `pollVoting` definido para qualquer pessoa, um visualizador sem usuário SSO vê "Entre para votar". Enquetes que ocultam os resultados até que você vote não recebem atualizações de contagem em tempo real; esses eleitores veem os novos números no próximo carregamento.

Para publicar uma enquete a partir da sua própria UI, passe um `CommentPollInput` para `postComment`:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)` e `sdk.getPollVoters(commentId:page:search:)` cobrem a votação e a lista de eleitores. As cores das enquetes estão listadas em [Cores de Enquetes](#theming-readme-generated).