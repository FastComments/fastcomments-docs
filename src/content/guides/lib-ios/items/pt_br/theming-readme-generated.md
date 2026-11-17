### Temas Predefinidos

Quatro temas predefinidos integrados estão disponíveis:

```swift
// Padrões do sistema
sdk.theme = FastCommentsTheme.default

// Cartões com sombras e cantos bem arredondados
sdk.theme = FastCommentsTheme.modern

// Plano, sem sombras, raio de canto pequeno, sem linhas de thread
sdk.theme = FastCommentsTheme.minimal

// Define todas as cores de ação para uma única cor de marca
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Estilos de Exibição de Comentários

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Lista plana com divisores (padrão)
theme.commentStyle = .card    // Cartões arredondados com sombras
theme.commentStyle = .bubble  // Estilo balão de chat
```

### Cores

Todas as propriedades de cor são opcionais. Valores não definidos recorrem a padrões sensatos do sistema.

```swift
var theme = FastCommentsTheme()

// Cores de marca
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
theme.commentSpacing = 4          // Pontos entre linhas de comentários
theme.nestingIndent = 20          // Pontos de recuo por nível de aninhamento
theme.avatarSize = 36             // Diâmetro do avatar para comentários raiz
theme.replyAvatarSize = 28        // Diâmetro do avatar para respostas aninhadas
```

### Efeitos Visuais

```swift
theme.showShadows = true          // Sombras sutis nos cartões
theme.showThreadLine = true       // Linha vertical conectando respostas aninhadas
theme.animateVotes = true         // Animação de mola nas mudanças de voto
```

### Aplicando Temas

Duas abordagens:

```swift
// Via ambiente SwiftUI (recomendado para hierarquia de views)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Diretamente no SDK
sdk.theme = theme
```

---


### Cores de Enquetes

As enquetes usam por padrão as cores do cartão de enquete do widget web, nos modos claro e escuro. Definir `primaryColor` também altera a cor de destaque e a barra da opção que você escolheu. Para defini-las diretamente:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Propriedade | Usada para |
|----------|----------|
| `pollAccentColor` | Borda e marca de seleção da opção escolhida, duração escolhida, "Seu voto foi registrado" |
| `pollAccentTextColor` | Marca de seleção e texto desenhados sobre a cor de destaque |
| `pollOptionBackgroundColor` | Fundo do chip de opção |
| `pollOptionPressedBackgroundColor` | Fundo do chip de opção enquanto pressionado |
| `pollBarColor` | Barra de resultado para opções que você não escolheu |
| `pollSelectedBarColor` | Barra de resultado para a opção que você escolheu |
| `pollBorderColor` | Marca da caixa de seleção e pílulas com contorno |
| `pollDangerColor` | Erros e "Remover enquete" |