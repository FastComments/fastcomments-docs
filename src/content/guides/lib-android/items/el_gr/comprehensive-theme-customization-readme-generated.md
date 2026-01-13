Όλα τα κουμπιά και τα στοιχεία διεπαφής χρήστη (UI) στο FastComments SDK μπορούν να θεματοποιηθούν. Χρησιμοποιήστε το `FastCommentsTheme.Builder` για πλήρη έλεγχο της επωνυμίας της εφαρμογής σας.

### Προγραμματική Θεματοποίηση (Συνιστάται)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Κουμπιά ενέργειας: Αποστολή, ψήφος, μενού, κουμπιά like/share
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Κουμπιά απάντησης: Κουμπιά απάντησης σχολίου  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Κουμπιά εναλλαγής: Κουμπιά εμφάνισης/απόκρυψης απαντήσεων
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Κουμπιά φόρτωσης περισσότερων: Κουμπιά σελιδοποίησης
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Εφαρμόστε το θέμα
sdk.setTheme(theme)
```

### Γρήγορη αντικατάσταση χρωμάτων

Παρακάμψτε τους πόρους χρωμάτων στο `colors.xml` της εφαρμογής σας για απλή προσαρμογή της επωνυμίας:

```xml
<!-- Στο res/values/colors.xml της εφαρμογής σας -->
<resources>
    <!-- Αλλάξτε όλα τα κύρια στοιχεία UI -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Ή προσαρμόστε συγκεκριμένους τύπους κουμπιών -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Κάλυψη Θεματοποιημένων Κουμπιών

**Κάθε κουμπί στο SDK υποστηρίζει θεματοποίηση:**
- Κουμπιά αποστολής, κουμπιά ψήφου, κουμπιά μενού, κουμπιά απάντησης
- Κουμπιά εμφάνισης/απόκρυψης απαντήσεων, κουμπιά φόρτωσης περισσότερων  
- Κουμπιά ενέργειας ροής (μου αρέσει, σχόλιο, κοινοποίηση)
- Κουμπιά διαλόγου (υποβολή, ακύρωση, αποθήκευση)
- Δυναμικά κουμπιά εργασιών στις δημοσιεύσεις ροής

Για λεπτομερή τεκμηρίωση θεματοποίησης, δείτε [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).