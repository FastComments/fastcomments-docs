Αυτή η βιβλιοθήκη αποτελεί μια πλήρη υλοποίηση για react-native του [FastComments](https://fastcomments.com).

Υποστηρίζει live commenting, chat, threads, emoticons, ειδοποιήσεις, SSO, skins, και πλήρη προσαρμογή με την παροχή ενός stylesheet αντικειμένου. Όλα τα assets
μπορούν επίσης να προσαρμοστούν, και υποστηρίζει την εναλλαγή διαφορετικών assets ανάλογα με τη σκοτεινή λειτουργία.

Το πλεονέκτημα αυτής της βιβλιοθήκης είναι ότι είναι πιο ευέλικτη από το `fastcomments-react-native` wrapper. Τα σχόλια αποδίδονται με native components αντί μέσα σε ένα webview.

Όλα εκτελούνται στο backend του FastComments, οπότε χρειάζεται μόνο να ενσωματώσετε το UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Δείτε [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) για περισσότερα παραδείγματα.

Προσθέστε ζωντανό chat στην υπάρχουσα εφαρμογή React Native, ή ακόμα και δημιουργήστε ένα κοινωνικό δίκτυο!