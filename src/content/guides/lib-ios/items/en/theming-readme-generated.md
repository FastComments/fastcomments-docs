### Theme Presets

Four built-in presets are available:

```swift
// System defaults
sdk.theme = FastCommentsTheme.default

// Cards with shadows and large rounded corners
sdk.theme = FastCommentsTheme.modern

// Flat, no shadows, small corner radius, no thread lines
sdk.theme = FastCommentsTheme.minimal

// Set all action colors to a single brand color
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Comment Display Styles

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Flat list with dividers (default)
theme.commentStyle = .card    // Rounded cards with shadows
theme.commentStyle = .bubble  // Chat bubble style
```

### Colors

All color properties are optional. Unset values fall back to sensible system defaults.

```swift
var theme = FastCommentsTheme()

// Brand colors
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Backgrounds
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Action buttons
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Votes
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Links
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Dialogs
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Input bar
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Other
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Typography

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Layout and Spacing

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Points between comment rows
theme.nestingIndent = 20          // Points of indentation per nesting level
theme.avatarSize = 36             // Avatar diameter for root comments
theme.replyAvatarSize = 28        // Avatar diameter for nested replies
```

### Visual Effects

```swift
theme.showShadows = true          // Subtle shadows on cards
theme.showThreadLine = true       // Vertical line connecting nested replies
theme.animateVotes = true         // Spring animation on vote changes
```

### Applying Themes

Two approaches:

```swift
// Via SwiftUI environment (recommended for view hierarchy)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Directly on the SDK
sdk.theme = theme
```

---