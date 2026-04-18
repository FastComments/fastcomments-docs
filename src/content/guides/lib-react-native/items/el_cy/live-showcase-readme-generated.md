---
Για να δείτε κάθε widget και flow να τρέχει τοπικά ενάντια στο δημόσιο `demo` tenant, κλωνοποιήστε το repo και εκτελέστε:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Το σημείο εισόδου του showcase είναι το `example/src/ShowcaseApp.tsx` — μια ενιαία εφαρμογή που εκθέτει όλα τα widgets, themes, και flows.

Ο στόχος `yarn web` χρησιμοποιεί `react-native-web` + `react-native-web-webview` (που αποδίδει το WebView ως iframe). Χρήσιμο για γρήγορους οπτικούς smoke tests σε έναν browser· native-only WebView APIs όπως `injectJavaScript` και `onShouldStartLoadWithRequest` δεν θα συμπεριφέρονται πλήρως στο web.
---