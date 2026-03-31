### Predefinições de Tema

Quatro predefinições integradas estão disponíveis:

```swift
// Padrões do sistema
sdk.theme = FastCommentsTheme.default

// Cartões com sombras e cantos grandes arredondados
sdk.theme = FastCommentsTheme.modern

// Plano, sem sombras, raio de canto pequeno, sem linhas de encadeamento
sdk.theme = FastCommentsTheme.minimal

// Define todas as cores de ação para uma única cor da marca
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Estilos de Exibição de Comentários

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Lista plana com divisores (padrão)
theme.commentStyle = .card    // Cartões arredondados com sombras
theme.commentStyle = .bubble  // Estilo de balão de conversa
```

### Cores

Todas as propriedades de cor são opcionais. Valores não definidos usarão os padrões sensíveis do sistema.

```swift
var theme = FastCommentsTheme()

// Cores da marca
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Fundos
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Botões de ação
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Votos
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Links
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Diálogos
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Barra de entrada
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Outros
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Tipografia

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Layout e Espaçamento

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Pontos entre linhas de comentário
theme.nestingIndent = 20          // Pontos de indentação por nível de aninhamento
theme.avatarSize = 36             // Diâmetro do avatar para comentários raiz
theme.replyAvatarSize = 28        // Diâmetro do avatar para respostas aninhadas
```

### Efeitos Visuais

```swift
theme.showShadows = true          // Sombras sutis nos cartões
theme.showThreadLine = true       // Linha vertical conectando respostas aninhadas
theme.animateVotes = true         // Animação com efeito de mola nas mudanças de voto
```

### Aplicando Temas

Duas abordagens:

```swift
// Via SwiftUI environment (recommended for view hierarchy)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Directly on the SDK
sdk.theme = theme
```

---
---