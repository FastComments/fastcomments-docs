Όταν τα Προφίλ Χρηστών ανοίγουν στο πλαίσιο του ιστότοπού σας (μέσω του widget σχολίων), οποιαδήποτε προσαρμοσμένα στυλ CSS έχετε εφαρμόσει στο widget FastComments εγχύονται αυτόματα στο παράθυρο προφίλ.

### Πώς λειτουργεί

Όταν ένας χρήστης κάνει κλικ σε ένα σύνδεσμο προφίλ από το widget σχολίων σας, ανοίγει ένα παράθυρο προφίλ με την κλάση `.fast-comments-profile`. Τα προσαρμοσμένα CSS του widget σας εγχύονται αυτόματα στην προβολή προφίλ. Αν ήδη έχετε στιλιζάρει το widget σχολίων σας, αυτά τα στυλ θα ισχύουν και για τα προφίλ.

### Κλάσεις CSS

Τα προφίλ FastComments χρησιμοποιούν μια αρχιτεκτονική CSS βασισμένη σε κλάσεις. Δεν χρησιμοποιεί προσαρμοσμένες ιδιότητες CSS.

Η κύρια σελίδα προφίλ χρησιμοποιεί το `.user-profile` ως ριζικό κοντέινερ. Το τμήμα κεφαλίδας είναι `.profile-header` με το `.profile-header-background` για την εικόνα φόντου. Το περιεχόμενο προφίλ βρίσκεται στο `.profile-content`.

Το avatar χρησιμοποιεί `.profile-avatar` και `.profile-avatar-wrapper`. Το όνομα του χρήστη είναι `.profile-name` και το κείμενο βιογραφίας είναι `.profile-bio`. Τα στατιστικά είναι στο `.profile-stats` με τα μεμονωμένα στατιστικά να χρησιμοποιούν `.stat`.

Οι σύνδεσμοι κοινωνικών δικτύων βρίσκονται στο `.profile-social-links` με μεμονωμένους συνδέσμους ως `.social-link`. Τα διακριτικά (badges) χρησιμοποιούν `.profile-badges` και `.badge`. Οι μπάρες προόδου των διακριτικών χρησιμοποιούν `.progress-outer` και `.progress-bar`.

Οι καρτέλες χρησιμοποιούν `.profile-tabs` για το κοντέινερ, `.tab` για τις μεμονωμένες καρτέλες, και `.tab.active` για την επιλεγμένη καρτέλα. Το περιεχόμενο καρτέλας χρησιμοποιεί `.tab-body` και `.tab-body.active`. Οι μετρητές ειδοποιήσεων στις καρτέλες χρησιμοποιούν `.tab .count`.

Οι ειδοποιήσεις χρησιμοποιούν `.notification` και οι συνομιλίες DM χρησιμοποιούν `.conversation`. Η ένδειξη σύνδεσης είναι `.activity-indicator` με `.activity-indicator.online` για την ενεργή κατάσταση. Οι μετρητές μη αναγνωσμένων χρησιμοποιούν `.unread-count`.

Το κοντέινερ του παραθύρου προφίλ είναι `.fast-comments-profile` με το `.fast-comments-profile-close` για το κουμπί κλεισίματος.

### Σκοτεινή λειτουργία

Η σκοτεινή λειτουργία χρησιμοποιεί τον τροποποιητή κλάσης `.dark` στο `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Παραδείγματα

**Κεφαλίδα:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Διακριτικά:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Καρτέλες:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Παράθυρο:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```