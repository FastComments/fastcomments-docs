---
Αυτή η βιβλιοθήκη είναι μια πλήρης υλοποίηση για React Native του [FastComments](https://fastcomments.com).

Υποστηρίζει ζωντανό σχολιασμό, chat, threads, emoticons, ειδοποιήσεις, SSO, skins, και πλήρη εξατομίκευση με την παράδοση ενός αντικειμένου stylesheet. Όλα τα assets
μπορούν επίσης να εξατομικευτούν, και υποστηρίζει την εναλλαγή διαφορετικών assets ανάλογα με το dark mode.

Το πλεονέκτημα αυτής της βιβλιοθήκης είναι ότι είναι πιο ευέλικτη από τον wrapper `fastcomments-react-native`. Τα σχόλια αποδίδονται με native components αντί μέσα σε webview. Σημείωση: το `react-native-webview` εξακολουθεί να απαιτείται ως transitive dependency του rich text editor (`@10play/tentap-editor`).

Όλα τρέχουν στο backend του FastComments, οπότε χρειάζεται μόνο να ενσωματώσετε το UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Δείτε το [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) για περισσότερα παραδείγματα.

Προσθέστε live chat στην υπάρχουσα εφαρμογή React Native σας, ή ακόμα και δημιουργήστε ένα κοινωνικό δίκτυο!
---