Αυτή η βιβλιοθήκη είναι μια πλήρης υλοποίηση για react-native του [FastComments](https://fastcomments.com).

Υποστηρίζει ζωντανά σχόλια, chat, νήματα (threads), emoticons, ειδοποιήσεις, SSO, skins, και πλήρη προσαρμογή δίνοντας ένα αντικείμενο stylesheet. Όλα τα assets
μπορούν επίσης να προσαρμοστούν, και υποστηρίζει την εναλλαγή διαφορετικών assets με βάση το dark mode.

Το πλεονέκτημα αυτής της βιβλιοθήκης είναι ότι είναι πιο ευέλικτη από το `fastcomments-react-native` wrapper. Τα σχόλια αποδίδονται με native components αντί μέσα σε ένα webview.

Όλα τρέχουν στο backend του FastComments, οπότε χρειάζεται μόνο να ενσωματώσετε το UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Δείτε το [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) για περισσότερα παραδείγματα.

Προσθέστε ζωντανή συνομιλία στην υπάρχουσα εφαρμογή React Native σας, ή ακόμα και δημιουργήστε ένα κοινωνικό δίκτυο!