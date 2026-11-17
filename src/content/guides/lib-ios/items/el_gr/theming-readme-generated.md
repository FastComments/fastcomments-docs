### Προεπιλογές Θεμάτων

Τέσσερα ενσωματωμένα presets είναι διαθέσιμα:

```swift
// Προεπιλογές συστήματος
sdk.theme = FastCommentsTheme.default

// Κάρτες με σκιές και μεγάλες στρογγυλεμένες γωνίες
sdk.theme = FastCommentsTheme.modern

// Επίπεδο, χωρίς σκιές, μικρή ακτίνα γωνιών, χωρίς γραμμές νημάτων
sdk.theme = FastCommentsTheme.minimal

// Ορισμός όλων των χρωμάτων ενεργειών σε ένα ενιαίο χρώμα brand
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Στυ Εμφάνισης Σχολίων

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Επίπεδη λίστα με διαχωριστικά (προεπιλογή)
theme.commentStyle = .card    // Στρογγυλεμένες κάρτες με σκιές
theme.commentStyle = .bubble  // Στυ φυσαλίδων συνομιλίας
```

### Χρώματα

Όλες οι ιδιότητες χρωμάτων είναι προαιρετικές. Οι μη ορισμένες τιμές επιστρέφουν σε λογικές προεπιλογές συστήματος.

```swift
var theme = FastCommentsTheme()

// Χρώματα brand
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

// Ψήφοι
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

### Διάταξη και Αποστάσεις

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Σημεία μεταξύ γραμμών σχολίων
theme.nestingIndent = 20          // Σημεία εσοχής ανά επίπεδο εμφώλευσης
theme.avatarSize = 36             // Διάμετρος avatar για ριζικά σχόλια
theme.replyAvatarSize = 28        // Διάμετρος avatar για εμφωλευμένες απαντήσεις
```

### Οπτικά Εφέ

```swift
theme.showShadows = true          // Διακριτικές σκιές στις κάρτες
theme.showThreadLine = true       // Κάθετη γραμμή που συνδέει εμφωλευμένες απαντήσεις
theme.animateVotes = true         // Εφέ spring στις αλλαγές ψήφων
```

### Εφαρμογή Θεμάτων

Δύο προσεγγίσεις:

```swift
// Μέσω περιβάλλοντος SwiftUI (προτεινόμενο για ιεραρχία προβολών)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Απευθείας στο SDK
sdk.theme = theme
```

---


### Χρώματα Ψηφοφοριών

Οι ψηφοφορίες χρησιμοποιούν από προεπιλογή τα χρώματα της κάρτας ψηφοφορίας του web widget, σε φωτεινή και σκοτεινή λειτουργία. Ο ορισμός του `primaryColor` αλλάζει επίσης το χρώμα έμφασης και τη μπάρα της επιλογής που επιλέξατε. Για να τα ορίσετε απευθείας:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Ιδιότητα | Χρησιμοποιείται για |
|----------|----------|
| `pollAccentColor` | Περίγραμμα και σήμα επιλεγμένης επιλογής, επιλεγμένη διάρκεια, "Η ψήφος σας καταχωρήθηκε" |
| `pollAccentTextColor` | Σήμα επιλογής και κείμενο σε χρώμα έμφασης |
| `pollOptionBackgroundColor` | Φόντο chip επιλογής |
| `pollOptionPressedBackgroundColor` | Φόντο chip επιλογής κατά το πάτημα |
| `pollBarColor` | Μπάρα αποτελεσμάτων για επιλογές που δεν επιλέξατε |
| `pollSelectedBarColor` | Μπάρα αποτελεσμάτων για την επιλογή που επιλέξατε |
| `pollBorderColor` | Σήμα checkbox και pills με περίγραμμα |
| `pollDangerColor` | Σφάλματα και "Αφαίρεση ψηφοφορίας" |