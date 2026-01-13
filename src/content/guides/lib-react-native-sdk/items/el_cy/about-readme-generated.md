---
Αυτή η βιβλιοθήκη είναι μια πλήρης υλοποίηση για react-native του [FastComments](https://fastcomments.com).

Υποστηρίζει ζωντανά σχόλια, chat, νήματα, emoticons, ειδοποιήσεις, SSO, θέματα (skins), και πλήρη προσαρμογή μέσω ενός αντικειμένου stylesheet. Όλα τα assets
μπορούν επίσης να προσαρμοστούν, και υποστηρίζει εναλλαγή διαφορετικών assets βάσει του σκοτεινού θέματος.

Το πλεονέκτημα αυτής της βιβλιοθήκης είναι ότι είναι πιο ευέλικτη και δεν απαιτεί webview, όπως ο wrapper `fastcomments-react-native`.

Όλα εκτελούνται στο backend του FastComments, οπότε χρειάζεται μόνο να ενσωματώσετε το UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Δείτε [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) για περισσότερα παραδείγματα.

Προσθέστε ζωντανό chat στην υπάρχουσα εφαρμογή React Native σας, ή ακόμα και φτιάξτε ένα κοινωνικό δίκτυο!
---