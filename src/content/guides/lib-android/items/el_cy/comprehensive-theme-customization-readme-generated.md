Όλα τα κουμπιά και τα στοιχεία διεπαφής χρήστη στο FastComments SDK μπορούν να προσαρμοστούν θεματικά. Χρησιμοποιήστε τον `FastCommentsTheme.Builder` για πλήρη έλεγχο του branding της εφαρμογής σας.

### Προγραμματική θεματοποίηση (Συνιστάται)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Κουμπιά ενέργειας: Αποστολή, ψήφος, μενού, κουμπιά like/share
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Κουμπιά απάντησης: Κουμπιά απάντησης σχολίου  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Κουμπιά εναλλαγής: Κουμπιά εμφάνισης/απόκρυψης απαντήσεων
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Κουμπιά 'φόρτωσε περισσότερα': Κουμπιά σελιδοποίησης
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Εφαρμόστε το θέμα
sdk.setTheme(theme)
```

### Γρήγορη αντικατάσταση χρωμάτων

Παρακάμψτε τους πόρους χρωμάτων στο `colors.xml` της εφαρμογής σας για εύκολο branding:

```xml
<!-- Στο res/values/colors.xml της εφαρμογής σας -->
<resources>
    <!-- Αλλάξτε όλα τα βασικά στοιχεία διεπαφής χρήστη -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Ή προσαρμόστε συγκεκριμένους τύπους κουμπιών -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Κάλυψη θεματικών κουμπιών

**Κάθε κουμπί στο SDK υποστηρίζει θεματοποίηση:**
- Κουμπιά αποστολής, κουμπιά ψήφου, κουμπιά μενού, κουμπιά απάντησης
- Κουμπιά εμφάνισης/απόκρυψης απαντήσεων, κουμπιά 'φόρτωσε περισσότερα'  
- Κουμπιά ενέργειας στο feed (μου αρέσει, σχόλιο, κοινοποίηση)
- Κουμπιά διαλόγου (υποβολή, ακύρωση, αποθήκευση)
- Δυναμικά κουμπιά εργασιών στις αναρτήσεις του feed

Για λεπτομερή τεκμηρίωση θεματοποίησης, δείτε [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).