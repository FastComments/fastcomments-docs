Αυτή η βιβλιοθήκη είναι μια πλήρης υλοποίηση για react-native του [FastComments](https://fastcomments.com).

Υποστηρίζει σχολιασμό σε πραγματικό χρόνο, chat, νήματα, emoticons, ειδοποιήσεις, SSO, θέματα (skins) και πλήρη προσαρμογή παρέχοντας ένα αντικείμενο stylesheet. Όλα τα assets μπορούν επίσης να προσαρμοστούν, και υποστηρίζεται η εναλλαγή διαφορετικών assets με βάση το dark mode.

Το πλεονέκτημα αυτής της βιβλιοθήκης είναι ότι είναι πιο ευέλικτη από το `fastcomments-react-native` wrapper. Τα σχόλια αποδίδονται με γηγενή components αντί μέσα σε ένα webview. Σημείωση: το `react-native-webview` εξακολουθεί να απαιτείται ως μεταβατική εξάρτηση του επεξεργαστή πλούσιου κειμένου (`@10play/tentap-editor`).

Όλα τρέχουν στο backend του FastComments, οπότε πρέπει μόνο να ενσωματώσετε το UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Δείτε [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) για περισσότερα παραδείγματα.

Προσθέστε ζωντανή συνομιλία στην υπάρχουσα εφαρμογή React Native σας, ή ακόμη και δημιουργήστε ένα κοινωνικό δίκτυο!