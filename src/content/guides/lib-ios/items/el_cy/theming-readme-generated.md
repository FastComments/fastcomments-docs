### Προεπιλογές Θέματος

Διατίθενται τέσσερις ενσωματωμένες προεπιλογές:

```swift
// Προεπιλογές συστήματος
sdk.theme = FastCommentsTheme.default

// Κάρτες με σκιές και μεγάλες στρογγυλεμένες γωνίες
sdk.theme = FastCommentsTheme.modern

// Επίπεδο, χωρίς σκιές, μικρή ακτίνα γωνίας, χωρίς γραμμές νήματος
sdk.theme = FastCommentsTheme.minimal

// Ορισμός όλων των χρωμάτων ενεργειών σε ένα μόνο χρώμα μάρκας
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Στυλ Εμφάνισης Σχολίων

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Επίπεδη λίστα με διαχωριστικά (προεπιλεγμένο)
theme.commentStyle = .card    // Στρογγυλεμένες κάρτες με σκιές
theme.commentStyle = .bubble  // Στυλ φυσαλίδας συνομιλίας
```

### Χρώματα

Όλες οι ιδιότητες χρωμάτων είναι προαιρετικές. Οι μη ορισμένες τιμές επανέρχονται στις λογικές προεπιλογές του συστήματος.

```swift
var theme = FastCommentsTheme()

// Χρώματα μάρκας
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Φόντα
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Κουμπιά ενεργειών
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Ψηφοφορίες
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Σύνδεσμοι
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Διάλογοι
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Γραμμή εισαγωγής
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Άλλα
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Τυπογραφία

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Διάταξη και Διάκενα

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Μονάδες μεταξύ σειρών σχολίων
theme.nestingIndent = 20          // Μονάδες εσοχής ανά επίπεδο εμφώλευσης
theme.avatarSize = 36             // Διάμετρος avatar για κύρια σχόλια
theme.replyAvatarSize = 28        // Διάμετρος avatar για εμφωλευμένες απαντήσεις
```

### Οπτικά Εφέ

```swift
theme.showShadows = true          // Διακριτικές σκιές στις κάρτες
theme.showThreadLine = true       // Κάθετη γραμμή που συνδέει εμφωλευμένες απαντήσεις
theme.animateVotes = true         // Ελαστική κινούμενη μεταβολή στις αλλαγές ψήφων
```

### Εφαρμογή Θεμάτων

Δύο τρόποι:

```swift
// Μέσω περιβάλλοντος SwiftUI (συνιστάται για την ιεραρχία προβολών)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Άμεσα στο SDK
sdk.theme = theme
```