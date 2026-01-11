Όταν τα Προφίλ Χρηστών ανοίγουν στο πλαίσιο του ιστότοπού σας (μέσω του comment widget), οποιαδήποτε προσαρμοσμένα στυλ CSS που έχετε εφαρμόσει στο FastComments widget σας εισάγονται αυτόματα στο παράθυρο προφίλ.

### Πώς Λειτουργεί

Όταν ένας χρήστης κάνει κλικ σε ένα σύνδεσμο προφίλ από το comment widget σας, ανοίγει ένα παράθυρο προφίλ με την κλάση `.fast-comments-profile`. Τα προσαρμοσμένα CSS του widget σας εισάγονται αυτόματα στην προβολή προφίλ. Εάν έχετε ήδη στυλιζάρει το comment widget σας, αυτά τα στυλ θα εφαρμοστούν και στα προφίλ.

### Κλάσεις CSS

Τα προφίλ του FastComments χρησιμοποιούν μια αρχιτεκτονική CSS βασισμένη σε κλάσεις. Δεν χρησιμοποιεί προσαρμοσμένες ιδιότητες CSS (custom properties).

Η κύρια σελίδα προφίλ χρησιμοποιεί το `.user-profile` ως το ριζικό δοχείο. Η ενότητα κεφαλίδας είναι `.profile-header` με `.profile-header-background` για την εικόνα φόντου. Το περιεχόμενο του προφίλ βρίσκεται στο `.profile-content`.

Το avatar χρησιμοποιεί `.profile-avatar` και `.profile-avatar-wrapper`. Το όνομα του χρήστη είναι `.profile-name` και το κείμενο βιογραφίας είναι `.profile-bio`. Τα στατιστικά είναι στο `.profile-stats` με μεμονωμένα στατιστικά που χρησιμοποιούν `.stat`.

Οι σύνδεσμοι κοινωνικών δικτύων βρίσκονται σε `.profile-social-links` με μεμονωμένους συνδέσμους ως `.social-link`. Τα διακριτικά (badges) χρησιμοποιούν `.profile-badges` και `.badge`. Οι μπάρες προόδου των διακριτικών χρησιμοποιούν `.progress-outer` και `.progress-bar`.

Οι καρτέλες χρησιμοποιούν `.profile-tabs` για το δοχείο, `.tab` για μεμονωμένες καρτέλες, και `.tab.active` για την επιλεγμένη καρτέλα. Το περιεχόμενο καρτέλας χρησιμοποιεί `.tab-body` και `.tab-body.active`. Οι μετρητές ειδοποιήσεων στις καρτέλες χρησιμοποιούν `.tab .count`.

Οι ειδοποιήσεις χρησιμοποιούν `.notification` και οι συνομιλίες DM χρησιμοποιούν `.conversation`. Η ένδειξη διαδικτυακής παρουσίας είναι `.activity-indicator` με `.activity-indicator.online` για την ενεργή κατάσταση. Οι μετρητές μη αναγνωσμένων χρησιμοποιούν `.unread-count`.

Το δοχείο του παραθύρου προφίλ είναι `.fast-comments-profile` με `.fast-comments-profile-close` για το κουμπί κλεισίματος.

### Σκοτεινή λειτουργία

Η σκοτεινή λειτουργία χρησιμοποιεί τον τροποποιητή κλάσης `.dark` στο `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Παραδείγματα

**Header:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Badges:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Tabs:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Modal:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```