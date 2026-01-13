Αυτή η βιβλιοθήκη είναι μια πλήρης υλοποίηση για react-native του [FastComments](https://fastcomments.com).

Υποστηρίζει ζωντανό σχολιασμό, chat, threads, emoticons, ειδοποιήσεις, SSO, θέματα (skins) και πλήρη παραμετροποίηση μέσω ενός αντικειμένου stylesheet. Όλα τα assets μπορούν επίσης να προσαρμοστούν, και υποστηρίζει εναλλαγή διαφορετικών assets ανάλογα με τη σκοτεινή λειτουργία (dark mode).

Το πλεονέκτημα αυτής της βιβλιοθήκης είναι ότι είναι πιο ευέλικτη και δεν απαιτεί webview, όπως ο wrapper `fastcomments-react-native`.

Όλα λειτουργούν στο backend του FastComments, οπότε χρειάζεται μόνο να ενσωματώσετε το UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Δείτε το [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) για περισσότερα παραδείγματα.

Προσθέστε ζωντανή συνομιλία στην υπάρχουσα εφαρμογή React Native σας, ή ακόμη και δημιουργήστε ένα κοινωνικό δίκτυο!