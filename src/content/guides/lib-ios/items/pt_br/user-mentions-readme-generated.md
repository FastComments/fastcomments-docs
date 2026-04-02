Pesquisar usuários para dar suporte ao autocompletar de @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Retorna [UserSearchResult] com userId, username, avatar, etc.
```

O `CommentInputBar` integrado lida com o autocompletar de @mention automaticamente.